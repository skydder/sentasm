use tokenizer::{Location, Tokenizer};
use data::{PREPOSITION, Preposition};

pub(crate) fn parse_preposition<'a>(tokenizer: &Tokenizer<'a>) -> Option<(Preposition<'a>, Location<'a>)> {
    let token = tokenizer.peek();
    let loc = tokenizer.get_location();

    if !token.is_identifier() {
        return None;
    }
    
    let prep_name = token.get_identifier().unwrap();

    for prep in PREPOSITION {
        if prep_name == *prep {
            tokenizer.next();
            return Some((Preposition(prep), loc));
        }
    }
    None
}