mod wrap;
use boa_engine::{JsValue, JsResult, Context};
use polywrap_wasm_rs::{subinvoke, wrap_debug_log};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use wrap::*;
mod eval;
use eval::{eval_and_parse, GlobalFun};
pub use wrap::{EvalResult, ArgsEval, ModuleTrait, Module};
use getrandom::register_custom_getrandom;

fn custom_getrandom(_: &mut [u8]) -> Result<(), getrandom::Error> {
    return Ok(());
}

register_custom_getrandom!(custom_getrandom);

impl ModuleTrait for Module {
    fn eval(args: ArgsEval) -> Result<EvalResult, String> {
        let result = eval_and_parse(&args.src, vec![], vec![
            GlobalFun{
                name: "__wrap_subinvoke".to_string(),
                function: subinvoke
            },
            GlobalFun{
                name: "__wrap_abort".to_string(),
                function: abort
            }
        ]);

        match result {
            Ok(result) => {
                let result = match result.1 {
                    Some(result) => {
                        result
                    },
                    None => result.0
                };
                Ok(EvalResult {
                    value: Some(result),
                    error: None
                })
            },
            Err(err) => Err(err)
        }
    }

    fn eval_with_globals(args: ArgsEvalWithGlobals) -> Result<EvalResult, String> {
        let result = eval_and_parse(&args.src, args.globals, vec![
            GlobalFun{
                name: "__wrap_subinvoke".to_string(),
                function: subinvoke
            },
            GlobalFun{
                name: "__wrap_getImplementations".to_string(),
                function: get_implementations
            },
            GlobalFun{
                name: "__wrap_abort".to_string(),
                function: abort
            },
            GlobalFun{
                name: "__wrap_debug_log".to_string(),
                function: debug_log
            }
        ]);

        match result {
            Ok(result) => {
                let result = match result.1 {
                    Some(result) => {
                        result
                    },
                    None => result.0
                };
                Ok(EvalResult {
                    value: Some(result),
                    error: None
                })
            },
            Err(err) => Err(err)
        }
    }
}

#[derive(Serialize, Deserialize)]
struct JsonResult {
    ok: bool,
    error: Option<String>,
    value: Option<Value>
}

fn js_value_from_value(val: Value, ctx: &mut Context<'_>) -> JsValue {
    let val = serde_json::to_value(&JsonResult {
        ok: true,
        error: None,
        value: Some(val),
    }).unwrap();

    JsValue::from_json(&val, ctx).unwrap()
}

fn js_value_from_error(err: String, ctx: &mut Context<'_>) -> JsValue {
    let val = serde_json::to_value(&JsonResult {
        ok: false,
        error: Some(err),
        value: None,
    }).unwrap();

    JsValue::from_json(&val, ctx).unwrap()
}

fn subinvoke(_: &JsValue, args: &[JsValue], ctx: &mut Context<'_>) -> JsResult<JsValue> {
    let uri = args.get(0).unwrap();
    let uri: String = uri.as_string().unwrap().to_std_string().unwrap();
    
    let method = args.get(1).unwrap();
    let method = method.as_string().unwrap().to_std_string().unwrap();

    let args = args.get(2).unwrap();
    let args: Vec<u8> = serde_json::from_value(args.to_json(ctx).unwrap()).unwrap();

    let result: Result<Vec<u8>, String> = subinvoke::wrap_subinvoke(
        &uri,
        &method,
        args,
    );

    let result = result.and_then(|result| {
        let result = msgpack_to_json(&result);

        serde_json::from_str(&result)
            .map(|json| {
                js_value_from_value(json, ctx)
            })
            .map_err(|e| e.to_string())
    }).or_else(|err| {
        Ok(js_value_from_error(err, ctx))
    });

    result
}

fn get_implementations(_: &JsValue, args: &[JsValue], ctx: &mut Context<'_>) -> JsResult<JsValue> {
  let uri = args.get(0).unwrap();
  let uri: String = uri.as_string().unwrap().to_std_string().unwrap();

  let result = polywrap_wasm_rs::get_implementations::wrap_get_implementations(
      &uri
  );
  let result_json = Value::Array(result.into_iter().map(|r| Value::String(r)).collect());

  let result = js_value_from_value(result_json, ctx);
  JsResult::Ok(result)
}

fn abort(_: &JsValue, args: &[JsValue], ctx: &mut Context<'_>) -> JsResult<JsValue> {
    let args = args.get(0).unwrap();
    let args = args.to_json(ctx).unwrap().to_string();

    panic!("{}", args);
}

fn debug_log(_: &JsValue, args: &[JsValue], ctx: &mut Context<'_>) -> JsResult<JsValue> {
    let arg = args.get(0).unwrap();
    let msg: String = arg.as_string().unwrap().to_std_string().unwrap();

    wrap_debug_log(&format!("{}", msg));

    JsResult::Ok(JsValue::Boolean(true))
}

pub fn msgpack_to_json(bytes: &[u8]) -> String {
    let value: rmpv::Value = rmp_serde::from_slice(&bytes).unwrap();
    serde_json::to_string(&value).unwrap()
}

#[cfg(test)]
mod tests {
    use polywrap_client::msgpack;
    use serde_json::json;

    use crate::{tests::test_utils::{load_wrap, get_client_with_module, invoke_client}, EvalResult};

    mod test_utils;

    // #[test]
    // fn sanity() {
    //     let (_manifest, module) = load_wrap("./bin");

    //     let client = get_client_with_module(&module);

    //     let result = invoke_client("mock/test", "eval", &msgpack::msgpack!({
    //         "src": "const temp = 'Hello world'; temp"
    //     }), &client);

    //     let result: EvalResult = rmp_serde::from_slice(&result).unwrap();

    //     assert_eq!(result.value.unwrap(), EvalResult {
    //         value: Some(json!("Hello world")),
    //         error: None
    //     }.value.unwrap());
    // }
}