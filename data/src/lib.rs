mod data_auto;
mod data;
mod parser;
mod tokenizer;


pub use data_auto::*;
pub use data::{
    Register, Data, DataSet, Memory, Immediate, Label, Keyword, Preposition, Verb, RegType
};

pub use tokenizer::{Loc, Tonkenizer};
pub use parser::{Code, PrepositionPhrases, Sentence};

pub type Result<T> = std::result::Result<T, ()>;

#[macro_export] macro_rules! emit_error_msg {
    ($msg:expr, $loc:expr) => {
        eprintln!("{}", format!("{}{}", $msg, $loc))
    };
}