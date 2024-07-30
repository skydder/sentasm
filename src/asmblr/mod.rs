use std::result;

mod tokenizer;
mod parser;
mod data;
// mod codegen;

pub use tokenizer::{Tonkenizer, Loc};
pub use data::{Data, DataSet, Memory};
use data::{Verb, Preposition, Keyword, Label, Register};
pub use parser::Code;
use parser::PrepositionPhrases;

pub type Result<T> = result::Result<T, ()>;