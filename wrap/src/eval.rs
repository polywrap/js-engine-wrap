use boa_engine::{NativeFunction, Context, module::ModuleLoader, native_function::NativeFunctionPointer, Source, JsString, JsResult, JsValue, property::Attribute};
use serde_json::Value;

use crate::wrap::global_var::GlobalVar;

pub fn eval_and_parse(src: &str, global_vars: Vec<GlobalVar>, global_funcs: Vec<GlobalFun>) -> Result<Value, String>  {
    let loader = &CustomModuleLoader::new().unwrap();
    let dyn_loader: &dyn ModuleLoader = loader;

    let mut ctx = &mut Context::builder().module_loader(dyn_loader).build().unwrap();

    for global in global_vars {
        let value = match serde_json::from_value(global.value) {
            Ok(json) => JsValue::from_json(&json, ctx).unwrap(),
            Err(err) => {
                let json = serde_json::to_string(&err.to_string()).unwrap();
                let json = serde_json::from_str(&json).unwrap();
                JsValue::from_json(&json, ctx).unwrap()
            }
        };

        ctx.register_global_property(global.name, value, Attribute::PERMANENT)
            .unwrap();
    }

    for global in global_funcs {
        ctx.register_global_callable(&global.name, 0, NativeFunction::from_fn_ptr(global.function))
            .unwrap();
    }

    let result = ctx.eval(Source::from_bytes(src.as_bytes()));

    let result = result
        .and_then(|x| x.to_json(&mut ctx))
        .map_err(|err| err.to_string());

    ctx.run_jobs();
    result
}

#[derive(Debug)]
pub struct CustomModuleLoader;

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

    use crate::{eval_and_parse, eval::GlobalFun};

    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    pub struct MockType {
        pub prop: String,
    }

    #[test]
    fn eval_null() {
        let src = "const temp = null; temp";
        
        let result = eval_and_parse(src, vec![], vec![]);

        let result = result.unwrap();

        let expected = json!(null);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn eval_string() {
        let src = "const temp = 'Hello world'; temp";
        
        let result = eval_and_parse(src, vec![], vec![]);

        let result = result.unwrap();

        let expected = json!("Hello world");
        
        assert_eq!(result, expected);
    }

    #[test]
    fn eval_bool() {
        let src = "const temp = true; temp";
          
        let result = eval_and_parse(src, vec![], vec![]);

        let result = result.unwrap();

        let expected = json!(true);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn eval_integer() {
      let src = "const temp = 10; temp";
          
      let result = eval_and_parse(src, vec![], vec![]);

      let result = result.unwrap();

      let expected = json!(10);
      
      assert_eq!(result, expected);
    }

    #[test]
    fn eval_rational() {
      let src = "const temp = 123.456; temp";
          
      let result = eval_and_parse(src, vec![], vec![]);

      let result = result.unwrap();

      let expected = json!(123.456);
      
      assert_eq!(result, expected);
    }

    #[test]
    fn eval_object() {
        let src = "({ prop: 'Hello world' });".to_string();

        let result: Result<serde_json::Value, String> = eval_and_parse(&src, vec![], vec![]);

        let result = result.unwrap();

        let result: MockType = serde_json::from_value(result).unwrap();

        let expected = MockType {
            prop: "Hello world".to_string()
        };
        
        assert_eq!(result, expected);
    }

    #[test]
    fn eval_global_function() {
        use boa_engine::{Context, JsValue, JsResult};
    
        // The global function to be registered
        fn global_fn(_: &JsValue, _args: &[JsValue], _ctx: &mut Context<'_>) -> JsResult<JsValue> {
            Ok(JsValue::new("global function invoked"))
        }
    
        // Wrap it in a GlobalFun struct
        let global = GlobalFun {
            name: "myGlobalFunction".to_string(),
            function: global_fn,
        };
    
        let src = "myGlobalFunction();";
    
        let result = eval_and_parse(src, vec![], vec![global]);
    
        let result = result.unwrap();
    
        let expected = json!("global function invoked");
    
        assert_eq!(result, expected);
    }
    
    #[test]
    fn eval_undefined_variable() {
        let src = "notDefinedVariable;";
    
        let result = eval_and_parse(src, vec![], vec![]);

        match result {
            Ok(_) => panic!("Expected error for undefined variable, but didn't get one"),
            Err(e) => assert!(e.contains("notDefinedVariable is not defined"), "Unexpected error message: {}", e),
        }
    }
}
