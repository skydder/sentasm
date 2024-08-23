use std::result;

mod codegen;
mod data;
mod parser;
mod tokenizer;

mod gen_gen;

pub use codegen::codegen;
pub use data::{Data, DataSet};
pub use parser::Code;
pub use tokenizer::{Loc, Tonkenizer};
pub type Result<T> = result::Result<T, ()>;

pub(crate) use data::{Label, Preposition, Verb, Register, Keyword, Immediate, Memory};
pub(crate) use parser::PrepositionPhrases;



#[macro_export] macro_rules! emit_error_msg {
    ($msg:expr, $loc:expr) => {
        eprintln!("{}", format!("{}{}", $msg, $loc))
    };
}