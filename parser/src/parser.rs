use tokenizer::{Token, Tokenizer};
use data::{Code, Data, DataSet, Label};

use crate::{parse_code, parse_define, parse_immediate, parse_keyword, parse_memory, parse_preposition, parse_register, parse_verb};

pub fn parser<'a>(tokenizer: &'a Tokenizer<'a>) -> Code<'a> {
    parse_code(tokenizer)
}

pub(crate) fn parse_data_set<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<DataSet<'a>> {
    if let Some((imm, loc)) = parse_immediate(tokenizer) {
        Some(DataSet::new_(Data::Immediate(imm), loc))
    } else if let Some((reg, loc)) = parse_register(tokenizer) {
        Some(DataSet::new_(Data::Register(reg), loc))
    } else if let Some((verb, loc)) = parse_verb(tokenizer) {
        Some(DataSet::new_(Data::Verb(verb), loc))
    } else if let Some((keyword, loc)) = parse_keyword(tokenizer) {
        Some(DataSet::new_(Data::Keyword(keyword), loc))
    } else if let Some((prep, loc)) = parse_preposition(tokenizer) {
        Some(DataSet::new_(Data::Preposition(prep), loc))
    } else if let Some((mem, loc)) = parse_memory(tokenizer) {
        Some(DataSet::new_(Data::Memory(mem), loc))
    } else if let Some((def, loc)) = parse_define(tokenizer) {
        Some(DataSet::new_(Data::Define(def), loc))
    } else { 
        match tokenizer.peek() {
            Token::Identifier(ident, location) => {
                tokenizer.next();
                Some(DataSet::new_(Data::Label(Label(ident)), location))
            },
            _ => None
        }
    }
}

#[test]
fn test_parser() {
    use tokenizer::Stream;
    let stream = Stream::new("move *(rax by 32bit) to rax", "test");
    let tokenizer = Tokenizer::new(&stream);
    eprintln!("{:?}", parse_code(&tokenizer));
}