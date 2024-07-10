pub mod codegen;
pub mod sentence;
pub mod token;
use core::fmt;

pub use codegen::codegen;
pub use sentence::Sentence;
pub use token::{Token, TokenLocation};

pub(crate) use sentence::{Object, Preposition, PrepositionPhrases, Verb, TokenInfo};

#[derive(Debug)]
pub enum AsmError<'a> {
    SyntaxError(TokenLocation<'a>, String),
    IOError(String)
}

impl<'a> fmt::Display for AsmError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SyntaxError(loc,msg ) => write!(f, "syntax error: {} ({})",msg, loc),
            Self::IOError(msg) => write!(f, "io error: {}", msg)
        }
        
    }
}
