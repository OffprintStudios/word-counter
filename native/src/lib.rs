//! A native module for counting the number of words in the body of work submitted to Offprint.
//! Defines a single `count_words()` function, which is exposed via NodeJS's NAPI to NodeJS consumers
//! as `countWords(string)`.

use neon::prelude::*;

use serde::de::IgnoredAny;
use serde::Deserialize;
use serde_json::Result;

use voca_rs::count;

#[derive(Deserialize, Debug)]
pub struct Delta {
    ops: Vec<DeltaOps>,
}

#[derive(Deserialize, Debug)]
pub struct DeltaOps {    
    insert: StrOrMap,
    attributes: Option<IgnoredAny>,
}

#[derive(Deserialize, Debug)]
// This serde attibute says "don't look for anything with this enum's names, just deserialize with the first thing that works".
// The final Unknown(IgnoreAny) will accept anything, but ignore and not deserialize it.
#[serde(untagged, rename_all = "camelCase")]
enum StrOrMap {
    Text(String),
    Map { child_object: IgnoredAny },
    Unknown(IgnoredAny),
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
    let mut string_builder = String::new();
    for chunk in work_content.ops.iter().filter_map(|x| match &x.insert {
        StrOrMap::Text(text) => Some(text),
        _ => None,
    }) {
        string_builder.push_str(&chunk);
    }
    let word_count = count::count_words(&string_builder, "");
    Ok(word_count as u32)
}

register_module!(mut m, {
    m.export_function("countWords", count_words)?;
    Ok(())
});
