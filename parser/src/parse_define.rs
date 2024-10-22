use tokenizer::{emit_error, Location, Token, Tokenizer};
use data::{DefItem, Define};

use crate::parse_number;

// Define = [DefNums | DefString]
// DefItems = (number,)* number
// DefItem = number | string

pub(crate) fn parse_define<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<(Define<'a>, Location<'a>)> {
    let token = tokenizer.peek();
    let loc = tokenizer.get_location();
    
    if !token.get_punctuator().is_some_and(|punc| punc == "[") {
        return None;
    }
    tokenizer.next();
    let def = if let Some(define) = parse_defitems(tokenizer) {
        eprintln!("{:?}", define);
        define
    } else {
        // error
        emit_error!(tokenizer.get_location(), "expected items of define, but found other.");
        todo!()
    };
    tokenizer.expect_punctuator("]");
    eprintln!("{:?}", def);
    Some((def, loc))
}

fn parse_defitems<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<Define> {
    let mut seq: Vec<DefItem> = Vec::new();
    while tokenizer.peek2().get_punctuator().is_some_and(|punc| punc == ",") || tokenizer.peek3().get_punctuator().is_some_and(|punc| punc == ",") {
        if let Some(item) = parse_defitem(tokenizer) {
            tokenizer.expect_punctuator(",");
            seq.push(item);
        } else {
            // error
            emit_error!(tokenizer.get_location(), "expected item of define, but found other.");
            todo!();
        }
    }
    if let Some(last_item) = parse_defitem(tokenizer) {
        seq.push(last_item);
    } else {
        // error
        eprintln!("should be error?");
        todo!()
    }
    Some(Define::_new(seq))
}

// ***This code looks familier!!
fn parse_defitem<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<DefItem> {
    let token = tokenizer.peek();
    match token {
        Token::Number(..) => {
            let imm = parse_number(&token).unwrap();
            tokenizer.next();
            Some(DefItem::Int(imm as i64))
        },
        Token::Punctuator("-", ..) => {
            tokenizer.next();
            if let Some(imm) = parse_number(&tokenizer.peek()) {
                tokenizer.next();
                Some(DefItem::Int(-(imm as i64)))
            } else {
                // error
                emit_error!(tokenizer.get_location(), "only number can come here, but other is here.");
                todo!()
            }
        },
        Token::String(s, ..) => {
            tokenizer.next();
            Some(DefItem::Str(s))
        }
        _ => None
    }
}