//! A native module for counting the number of words in the body of work submitted to Offprint.
//! Defines a single `count_words()` function, which is exposed via NodeJS's NAPI to NodeJS consumers
//! as `countWords(string)`.

use neon::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::{value::RawValue, Result, Value};
use voca_rs::count;

#[derive(Serialize, Deserialize, Debug)]
pub struct Delta<'a> {
    #[serde(borrow)]
    ops: Vec<DeltaOps<'a>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeltaOps<'a> {
    insert: String,
    #[serde(borrow)]
    attributes: Option<&'a RawValue>,
}

fn count_words(mut ctx: FunctionContext) -> JsResult<JsNumber> {
    let body_text = ctx.argument::<JsString>(0)?.value();
    match try_count_words(&body_text) {
        Ok(count) => Ok(ctx.number(count)),
        Err(_e) => Err(neon::result::Throw)
    }
}

pub fn try_count_words(text: &str) -> Result<u32> {
    let work_content: Delta = serde_json::from_str(text)?;
    let mut string_builder = String::new();
    for chunk in work_content.ops.iter().map(|x| &x.insert) {
        string_builder.push_str(chunk);
    }
    let word_count = count::count_words(&string_builder, "");
    Ok(word_count as u32)
}

register_module!(mut m, {
    m.export_function("countWords", count_words)?;
    Ok(())
});
