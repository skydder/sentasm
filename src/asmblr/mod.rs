use std::result;

mod data;
mod parser;
mod tokenizer;
mod codegen;

pub use data::{Data, DataSet};
pub(crate) use data::{Keyword, Label, Preposition, Register, Verb, Memory};
pub use parser::Code;
pub(crate) use parser::PrepositionPhrases;
pub use tokenizer::{Loc, Tonkenizer};
pub use codegen::codegen;

pub type Result<T> = result::Result<T, ()>;
