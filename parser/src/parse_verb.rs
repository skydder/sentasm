use tokenizer::{Tokenizer, Location};
use data::{PSEUDO, VERB, Verb};

pub(crate)fn parse_verb<'a>(tokenizer: &Tokenizer<'a>) -> Option<(Verb<'a>, Location<'a>)>{
    let token = tokenizer.peek();
    let loc = tokenizer.get_location();

    if !token.is_identifier() {
        return None;
    }

    let verb_name = token.get_identifier().unwrap();

    for verb in VERB {
        if verb_name == *verb {
            tokenizer.next();
            return Some((Verb(verb), loc));
        }
    }

    for verb in PSEUDO {
        if verb_name == *verb {
            tokenizer.next();
            return Some((Verb(verb), loc));
        }
    }
    None
}