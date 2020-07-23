//! A native module for counting the number of words in the body of work submitted to Offprint.
//! Defines a single `count_words()` function, which is exposed via NodeJS's NAPI to NodeJS consumers
//! as `countWords(string)`.

use neon::prelude::*;

use neon_serde::export;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

export! {
    fn count_words(delta_body: String) -> Result<u32, &'static str> {
        let body_content: Delta = match serde_json::from_str(&delta_body) {
            Ok(delta) => delta,
            Err(_) => return Err("Could not parse the delta body.")
        };
        let all_text = body_content
            .ops
            .iter()
            .filter(|x| x.insert.is_string())
            .map(|x| x.insert.as_str().unwrap())
            .collect::<String>();
        
        let word_count = count::count_words(&all_text, "");
        Ok(word_count as u32)
    }
}

/*register_module!(mut cx, {
    cx.export_function("hello", hello)
});*/