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
