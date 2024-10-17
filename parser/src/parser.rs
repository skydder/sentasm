use tokenizer::Tokenizer;
use data::{DataSet, Loc, Data};

use crate::{parse_register, parse_immediate, parse_verb, parse_keyword, parse_preposition};

pub fn parser() {

}

pub fn parse_data_set<'a>(tokenizer: &Tokenizer<'a>) -> Option<DataSet<'a>> {
    if let Some((imm, loc)) = parse_immediate(tokenizer) {
        Some(DataSet::new_(Data::Immediate(imm), loc))
    } else if let Some((reg, loc)) = parse_register(tokenizer) {
        Some(DataSet::new_(Data::Register(reg), loc))
    } else if let Some((verb, loc)) = parse_verb(tokenizer) {
        Some(DataSet::new_(Data::Verb(verb), loc))
    } else if let Some((keyword, loc)) = parse_keyword(tokenizer) {
        Some(DataSet::new_(Data::Keyword(keyword), loc))
    } else if let Some((prep, loc)) = parse_preposition(tokenizer) {
        Some(DataSet::new_(Data::Prepositon(prep), loc))
    } else {
        todo!()
    }
}