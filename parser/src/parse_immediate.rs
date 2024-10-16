use data::Immediate;
use tokenizer::{Location, Token, Tokenizer};

pub(crate) fn parse_immediate<'a>(tokenizer: &Tokenizer<'a>) -> Option<(Immediate, Location<'a>)> {
    let token = tokenizer.peek();
    let loc = tokenizer.get_location();
    
    match token {
        Token::Number(_, _) => {
            let imm = parse_number(&token);
            tokenizer.next();
            imm.map_or(None, |i| Some((i, loc)))
        },
        Token::Punctuator("-", _) => {
            let imm = parse_number(&tokenizer.peek2());
            if imm.is_some() {
                let i = imm.unwrap();
                tokenizer.next();
                tokenizer.next();
                Some((Immediate(i.0, i.1, true), loc))
            } else {
                None
            }
        },
        _ => None
    }
}

pub (crate) fn parse_number(token: &Token) -> Option<Immediate> {
    match token {
        Token::Number(i, _) => {
            Some(Immediate(*i as u64, 64, false))
        },
        _ => None
    }
}