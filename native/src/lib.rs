//! A native module for counting the number of words in the body of work submitted to Offprint.
//! Defines a single `count_words()` function, which is exposed via NodeJS's NAPI to NodeJS consumers
//! as `countWords(string)`.

use neon::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use voca_rs::count;

#[derive(Serialize, Deserialize, Debug)]
pub struct Delta {
    ops: Vec<DeltaOps>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeltaOps {
    attributes: Option<Value>,
    insert: Value
}

fn count_words(mut ctx: FunctionContext) -> JsResult<JsNumber> {
    let body_text = ctx.argument::<JsString>(0)?.value();
    match try_count_words(&body_text) {
        Ok(count) => Ok(ctx.number(count)),
        Err(_e) => Err(neon::result::Throw)
    }
}

fn try_count_words(text: &str) -> Result<u32> {
    let work_content: Delta = serde_json::from_str(text)?;
    let all_text = work_content
        .ops
        .iter()
        .filter(|x| x.insert.is_string())
        .map(|x| x.insert.as_str().unwrap())
        .collect::<String>();
    let word_count = count::count_words(&all_text, "");
    Ok(word_count as u32)
}

register_module!(mut m, {
    m.export_function("countWords", count_words)?;
    Ok(())
});