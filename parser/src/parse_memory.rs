use tokenizer::{Location, Tokenizer};
use data::Memory;

use crate::{parse_register, parse_number};

pub(crate) fn parse_memory<'a>(tokenizer: &Tokenizer<'a>) -> Option<(Memory<'a>, Location<'a>)> {
    let token = tokenizer.peek();
    let loc = tokenizer.get_location();
    
    if !token.get_punctuator().is_some_and(|punc| punc == "*(") {
        return None;
    }
    todo!();
}