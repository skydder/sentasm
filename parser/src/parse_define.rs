use tokenizer::{Location, Token, Tokenizer};
use data::Define_;

use crate::parse_number;

// Define = [DefNums | DefString]
// DefNums = (number,)* number
// DefString = string 

pub(crate) fn parse_define<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<(Define_<'a>, Location<'a>)> {
    let token = tokenizer.peek();
    let loc = tokenizer.get_location();
    
    if !token.get_punctuator().is_some_and(|punc| punc == "[") {
        return None;
    }
    tokenizer.next();
    let def = if let Some(defstring) = parse_defstring(tokenizer) {
        defstring
    } else if let Some(defnums) = parse_defnums(tokenizer) {
        defnums
    } else {
        // error
        todo!()
    };
    tokenizer.expect_punctuator("]");
    Some((def, loc))
}

fn parse_defstring<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<Define_<'a>> {
    match tokenizer.peek() {
        Token::String(s, ..) => {
            tokenizer.next();
            Some(Define_::String(s))
        },
        _ => None
    }
}

fn parse_defnums<'a>(tokenizer: &Tokenizer<'a>) -> Option<Define_<'a>> {
    if !tokenizer.is_number() {
        return None;
    }
    let mut seq: Vec<i64> = Vec::new();
    while tokenizer.peek2().get_punctuator().is_some_and(|punc| punc == ",") || tokenizer.peek().get_punctuator().is_some_and(|punc| punc == "_") {
        match parse_defnum(tokenizer) {
            Some(imm) => {
                tokenizer.expect_punctuator(",");
                seq.push(imm);
            },
            None => {
                // error
                todo!()
            }
        }
    }
    if let Some(last) = parse_defnum(tokenizer) {
        seq.push(last);
    } else {
        // error
        todo!()
    }
    Some(Define_::Number(seq))
}

// This code looks familier!!
fn parse_defnum<'a>(tokenizer: &Tokenizer<'a>) -> Option<i64> {
    let token = tokenizer.next();
    match token {
        Token::Number(..) => {
            let imm = parse_number(&token).unwrap();
            tokenizer.next();
            Some(imm as i64)
        },
        Token::Punctuator("-", ..) => {
            tokenizer.next();
            if let Some(imm) = parse_number(&tokenizer.peek()) {
                tokenizer.next();
                Some(-(imm as i64))
            } else {
                // error
                todo!()
            }
        },
        _ => None
    }
}