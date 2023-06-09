use boa_engine::{NativeFunction, Context, module::ModuleLoader, native_function::NativeFunctionPointer, Source, JsString, JsResult};
use serde_json::Value;

/// Evaluate the given ECMAScript code.
pub fn eval_and_parse(src: &str, globals: Vec<GlobalFun>) -> Result<Option<Value>, String>  {
  let loader = &CustomModuleLoader::new().unwrap();
  let dyn_loader: &dyn ModuleLoader = loader;

  // Just need to cast to a `ModuleLoader` before passing it to the builder.
  let mut ctx = &mut Context::builder().module_loader(dyn_loader).build().unwrap();

  for global in globals {
      ctx.register_global_callable(&global.name, 0, NativeFunction::from_fn_ptr(global.function))
          .unwrap();
  }

  let result = ctx.eval(Source::from_bytes(src.as_bytes()));

  let val = result.map_err(|err| err.to_string())?.to_json(&mut ctx).unwrap();
  Ok(Some(val))
}

#[derive(Debug)]
pub struct CustomModuleLoader {
}

impl CustomModuleLoader {
    pub fn new() -> JsResult<Self> {
        Ok(Self {
        })
    }
}

impl ModuleLoader for CustomModuleLoader {
    fn load_imported_module(
        &self,
        _referrer: boa_engine::module::Referrer,
        _specifier: JsString,
        _finish_load: Box<dyn FnOnce(JsResult<boa_engine::Module>, &mut Context<'_>)>,
        _context: &mut Context<'_>,
    ) {
    }
}

pub struct GlobalFun {
    pub name: String,
    pub function: NativeFunctionPointer,
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::eval_and_parse;

    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    pub struct MockType {
        pub prop: String,
    }

    #[test]
    fn eval_null() {
        let src = "const temp = null; temp";
        
        let result = eval_and_parse(src, vec![]);

        let result = result.unwrap().unwrap();

        let expected = json!(null);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn eval_string() {
        let src = "const temp = 'Hello world'; temp";
        
        let result = eval_and_parse(src, vec![]);

        let result = result.unwrap().unwrap();

        let expected = json!("Hello world");
        
        assert_eq!(result, expected);
    }

    #[test]
    fn eval_bool() {
        let src = "const temp = true; temp";
          
        let result = eval_and_parse(src, vec![]);

        let result = result.unwrap().unwrap();

        let expected = json!(true);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn eval_integer() {
      let src = "const temp = 10; temp";
          
      let result = eval_and_parse(src, vec![]);

      let result = result.unwrap().unwrap();

      let expected = json!(10);
      
      assert_eq!(result, expected);
    }

    #[test]
    fn eval_rational() {
      let src = "const temp = 123.456; temp";
          
      let result = eval_and_parse(src, vec![]);

      let result = result.unwrap().unwrap();

      let expected = json!(123.456);
      
      assert_eq!(result, expected);
    }

    #[test]
    fn eval_object() {
        let src = "({ prop: 'Hello world' });".to_string();

        let result: Result<Option<serde_json::Value>, String> = eval_and_parse(&src, vec![]);

        let result = result.unwrap().unwrap();

        let result: MockType = serde_json::from_value(result).unwrap();

        let expected = MockType {
            prop: "Hello world".to_string()
        };
        
        assert_eq!(result, expected);
    }

    #[test]
    fn eval_global_function() {
      //TODO: Implement this test
    }

    #[test]
    fn eval_undefined_variable() {
      //TODO: Implement this test
    }
}
