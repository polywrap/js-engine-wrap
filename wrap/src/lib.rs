mod wrap;
use boa_engine::{JsValue, JsResult, Context};
use polywrap_wasm_rs::subinvoke;
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
                name: "subinvoke".to_string(),
                function: subinvoke
            }
        ]);

        match result {
            Ok(result) => {
                Ok(EvalResult {
                    value: Some(result),
                    error: None
                })
            },
            Err(err) => {
                Ok(EvalResult {
                    value: None,
                    error: Some(err)
                })
            }
        }
    }

    fn eval_with_globals(args: ArgsEvalWithGlobals) -> Result<EvalResult, String> {
        let result = eval_and_parse(&args.src, args.globals, vec![
            GlobalFun{
                name: "subinvoke".to_string(),
                function: subinvoke
            }
        ]);

        match result {
            Ok(result) => {
                Ok(EvalResult {
                    value: Some(result),
                    error: None
                })
            },
            Err(err) => {
                Ok(EvalResult {
                    value: None,
                    error: Some(err)
                })
            }
        }
    }
}


fn subinvoke(_: &JsValue, args: &[JsValue], ctx: &mut Context<'_>) -> JsResult<JsValue> {
    let uri = args.get(0).unwrap();
    let uri: String = uri.as_string().unwrap().to_std_string().unwrap();
    
    let method = args.get(1).unwrap();
    let method = method.as_string().unwrap().to_std_string().unwrap();

    let args = args.get(2).unwrap();
    let args = args.to_json(ctx).unwrap().to_string();
    let args = json_to_msgpack(&args);

    let result: Result<Vec<u8>, String> = subinvoke::wrap_subinvoke(
        &uri,
        &method,
        args,
    );

    let result = match result {
        Ok(result) => msgpack_to_json(&result),
        Err(err) => {
            serde_json::to_string(&err).unwrap()
        }
    };

    let result = match serde_json::from_str(&result) {
        Ok(json) => JsValue::from_json(&json, ctx).unwrap(),
        Err(err) => {
            let json = serde_json::to_string(&err.to_string()).unwrap();
            let json = serde_json::from_str(&json).unwrap();
            JsValue::from_json(&json, ctx).unwrap()
        }
    };

    Ok(result)
}

pub fn msgpack_to_json(bytes: &[u8]) -> String {
    let value: rmpv::Value = rmp_serde::from_slice(&bytes).unwrap();
    serde_json::to_string(&value).unwrap()
}

pub fn json_to_msgpack(string: &str) -> Vec<u8> {
    let value: serde_json::Value = serde_json::from_str(string).unwrap();
    rmp_serde::encode::to_vec(&value).unwrap()
}

#[cfg(test)]
mod tests {
    use polywrap_client::msgpack;
    use serde_json::json;

    use crate::{tests::test_utils::{load_wrap, get_client_with_module, invoke_client}, EvalResult};

    mod test_utils;

    #[test]
    fn sanity() {
        let (_manifest, module) = load_wrap("./bin");

        let client = get_client_with_module(&module);

        let result = invoke_client("mock/test", "eval", &msgpack::msgpack!({
            "src": "const temp = 'Hello world'; temp"
        }), &client);

        let result: EvalResult = rmp_serde::from_slice(&result).unwrap();

        assert_eq!(result.value.unwrap(), EvalResult {
            value: Some(json!("Hello world")),
            error: None
        }.value.unwrap());
    }
}