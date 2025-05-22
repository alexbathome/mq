use anyhow::{Result, anyhow};
use jaq_core::load::{Arena, File, Loader};
use jaq_core::{Compiler, Ctx, RcIter};
use jaq_json::Val;
use serde_json::Value;

/// Run a JQ-like filter over JSON objects and return the resulting JSON values.
pub fn run_query(input: Vec<Value>, filter_code: &str) -> Result<Vec<Val>> {
    let program = File {
        code: filter_code,
        path: (),
    };

    let loader = Loader::new(jaq_std::defs().chain(jaq_json::defs()));
    let arena = Arena::default();

    let modules = loader.load(&arena, program).unwrap();

    let filter = Compiler::default()
        .with_funs(jaq_std::funs().chain(jaq_json::funs()))
        .compile(modules)
        .unwrap();

    let empty = RcIter::new(core::iter::empty());

    let v: Value = match input.len() {
        0..=1 => input.first().cloned().unwrap_or_default(),
        _ => serde_json::Value::Array(input).into(),
    };

    let out: Vec<Val> = filter
        .run((Ctx::new([], &empty), Val::from(v)))
        .map(|res| res.map_err(|e| anyhow!("Filter execution error: {:?}", e))) // Convert errors to anyhow::Error
        .collect::<Result<Vec<_>, _>>()?; // Collect into Vec<Val> or return the first error
    Ok(out)
}

#[cfg(test)]
mod tests {
    use jaq_json::Val;
    use serde_json::json;

    use super::run_query;

    #[test]
    fn test_filter_works() {
        let input = json!({"a": 1, "b": 2});

        println!("json data {:?}", input);
        let f = ".b";

        let out = run_query(vec![input], f);
        assert_eq!(*out.unwrap().first().unwrap(), Val::from(json!(2)));
    }
}
