mod wrap;
use wrap::*;
mod eval;
use eval::eval_and_parse;
pub use wrap::{EvalResult, ArgsEval, ModuleTrait, Module};
use getrandom::register_custom_getrandom;

fn custom_getrandom(_: &mut [u8]) -> Result<(), getrandom::Error> {
    return Ok(());
}

register_custom_getrandom!(custom_getrandom);

impl ModuleTrait for Module {
    fn eval(args: ArgsEval) -> Result<EvalResult, String> {
        let result = eval_and_parse(&args.src, vec![
        ])?;

        Ok(EvalResult {
            value: Some(result),
            error: None
        })
    }
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