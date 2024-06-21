pub mod codegen;
pub mod sentence;
pub mod token;
pub use codegen::codegen;
pub use sentence::Sentence;
pub use token::Token;

pub(crate) use sentence::{Object, Preposition, PrepositionPhrases, Verb};

#[derive(Debug)]
pub enum AsmError {
    SyntaxError(String),
}

pub trait Parse {
    fn parse(token: &mut Token) -> Result<Self, AsmError>
    where
        Self: Sized;
}
