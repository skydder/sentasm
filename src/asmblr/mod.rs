use std::result;

mod data;
mod parser;
mod tokenizer;
// mod codegen;

pub use data::{Data, DataSet, Memory};
use data::{Keyword, Label, Preposition, Register, Verb};
pub use parser::Code;
use parser::PrepositionPhrases;
pub use tokenizer::{Loc, Tonkenizer};

pub type Result<T> = result::Result<T, ()>;
