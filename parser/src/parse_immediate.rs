use data::Immediate;
use tokenizer::{Token, Tokenizer};

pub(crate) fn parse_immediate(tokenizer: &Tokenizer) -> Option<Immediate> {
    match tokenizer.peek() {
        Token::Number(i, _) => todo!(),
        Token::Punctuator("-", _) => todo!(),
        _ => None
    }
}