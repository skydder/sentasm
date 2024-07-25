use std::result;

mod tokenizer;
// mod parser;
mod data;
// mod codegen;

pub use tokenizer::{Tonkenizer, Loc};
pub use data::{Data, DataSet};
use data::{Verb, Preposition, Keyword, Label, Memory};
// use parser::{Code, PrepositionPhrases};

pub type Result<T> = result::Result<T, ()>;