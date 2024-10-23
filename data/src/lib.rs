mod data_auto;
mod data;
mod parser;


pub use data_auto::*;
pub use data::{
    Register, Data, DataSet, Memory, Immediate, Label, Keyword, Preposition, Verb, RegType, DefItem, Define 
};

pub use parser::{Code, PrepositionPhrases, Sentence, PrepositionObject};

pub type Result<T> = std::result::Result<T, ()>;

#[macro_export] macro_rules! emit_error_msg {
    ($msg:expr, $loc:expr) => {
        eprintln!("{}", format!("{}{}", $msg, $loc))
    };
}