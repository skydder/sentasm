use std::result;

mod codegen;
mod data;
mod parser;
mod tokenizer;

pub use codegen::codegen;
pub use data::{Data, DataSet};
pub(crate) use data::{Keyword, Label, Memory, Preposition, Register, Verb};
pub use parser::Code;
pub(crate) use parser::PrepositionPhrases;
pub use tokenizer::{Loc, Tonkenizer};

pub type Result<T> = result::Result<T, ()>;
