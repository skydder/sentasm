use std::{collections::HashMap, process::exit};

use macros::match_data2;
use tokenizer::{emit_error, Tokenizer};
use data::{Code, Data, DataSet, Label, Preposition, PrepositionObject, Sentence};

use crate::{parse_data_set, parse_prep_phrases, parse_verb};


pub(crate) fn parse_code<'a>(tokenizer: &'a Tokenizer<'a>) -> Code<'a> {
    if let Some(sentence) = parse_sentence(tokenizer) {
        tokenizer.expect_end_of_line();
        Code::Sentence(sentence)
    } else if let Some(section) = parse_section(tokenizer) { 
        tokenizer.expect_end_of_line();
        Code::Section(section)
    } else if let Some(def) = parse_labeldef(tokenizer) {
        tokenizer.expect_end_of_line();
        Code::LabelDef(def)
    } else if tokenizer.is_end_of_line() {
        tokenizer.expect_end_of_line();
        Code::NullStmt
    } else {
        // error
        emit_error!(tokenizer.get_location(), "unexpected syntax");
        exit(1);
    }
}

fn parse_sentence<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<Sentence<'a>> {
    let _verb = parse_verb(tokenizer);
    if _verb.is_none() {
        return None;
    }
    let (verb, location ) = _verb.unwrap();
    let mut data: HashMap<Preposition<'a>, PrepositionObject<'a>> = HashMap::new();
    let loc = tokenizer.get_location();
    let object = parse_data_set(tokenizer);
    match object {
        Some(match_data2!(Preposition, prep, location)) => {
            if let Some(obj) = parse_data_set(tokenizer) {
                data.insert(prep, PrepositionObject::new(obj, location));
            } else {
                // error
                emit_error!(location, "expected object after this preposition, but could not find it");
                exit(1);
            }
        },
        None => (),
        _ => {
            data.insert(Preposition("obj"), PrepositionObject::new(object.unwrap(), loc));
        }
    }
    let prep_phrases = parse_prep_phrases(tokenizer, data); 
    Some(Sentence::new(verb, location, prep_phrases))
}

fn parse_section<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<Label<'a>> {
    if !tokenizer.peek().get_punctuator().is_some_and(|punc| punc == "@") {
        return None;
    }
    tokenizer.next();
    match parse_data_set(tokenizer) {
        Some(match_data2!(Label, label)) => {
            Some(label)
        },
        other => {
            // error
            let loc = if let Some(data) = other {
                data.location
            } else {
                tokenizer.get_location()
            };
            emit_error!(loc, "only label can come here, but other is here.");
            exit(1);
        }
    }
}

fn parse_labeldef<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<Label<'a>> {
    if !tokenizer.peek().get_punctuator().is_some_and(|punc| punc == "#") {
        return None;
    }
    tokenizer.next();
    match parse_data_set(tokenizer) {
        Some(match_data2!(Label, label)) => {
            Some(label)
        },
        other => {
            // error
            let loc = if let Some(data) = other {
                data.location
            } else {
                tokenizer.get_location()
            };
            emit_error!(loc, "only label can come here, but other is here.");
            exit(1);
        }
    }
}