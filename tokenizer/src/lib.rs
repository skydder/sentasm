mod tokenizer;

pub use tokenizer::{Location, Tokenizer, Token, Stream, StreamInfo};

pub const PUNCTUATOR: &[&'static str] = & ["-", "+", "*", "(", ")", "@[", "]", "[", ];