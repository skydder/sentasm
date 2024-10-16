use tokenizer::{Location, Tokenizer};
use data::{KEYWORD, Keyword};

pub(crate) fn parse_keyword<'a>(tokenizer: &Tokenizer<'a>) -> Option<(Keyword<'a>, Location<'a>)> {
    let token = tokenizer.peek();
    let loc = tokenizer.get_location();

    if !token.is_identifier() {
        return None;
    }

    let keyword_name = token.get_identifier().unwrap();

    for keyword in KEYWORD {
        if keyword_name == *keyword {
            tokenizer.next();
            return Some((Keyword(keyword), loc));
        }
    }
    None
}