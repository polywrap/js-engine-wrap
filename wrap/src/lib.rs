mod wrap;
use wrap::*;
mod eval;
use eval::eval_and_parse;
use wrap::{EvalResult, ArgsEval, ModuleTrait, Module};
use getrandom::register_custom_getrandom;

fn custom_getrandom(_: &mut [u8]) -> Result<(), getrandom::Error> {
    return Ok(());
}

register_custom_getrandom!(custom_getrandom);

impl ModuleTrait for Module {
    fn eval(args: ArgsEval) -> Result<EvalResult, String> {
        let result = eval_and_parse(&args.src, vec![
        ]);

        let result = result?.unwrap();

        let result = EvalResult {
            value: Some(result),
            error: None
        };

        Ok(result)
    }
}
