pub mod sentence;
pub mod token;
pub use sentence::Sentence;
pub use token::Token;

#[derive(Debug)]
pub enum AsmError {
    SyntaxError(String),
}

pub trait Parse {
    fn parse(token: &mut Token) -> Result<Self, AsmError>
    where
        Self: Sized;
}
