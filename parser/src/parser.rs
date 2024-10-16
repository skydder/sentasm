use tokenizer::Tokenizer;
use data::DataSet;

use crate::{parse_register, parse_immediate, parse_verb, parse_keyword, parse_preposition};

pub fn parser() {

}

pub fn parse_data_set<'a>(tokenizer: &Tokenizer) -> Option<DataSet<'a>> {
    if let Some((imm, loc)) = parse_immediate(tokenizer) {
        todo!()
    } else if let Some((reg, loc)) = parse_register(tokenizer) {
        todo!()
    } else if let Some((verb, loc)) = parse_verb(tokenizer) {
        todo!()
    } else if let Some((keyword, loc)) = parse_keyword(tokenizer) {
        todo!()
    } else if let Some((prep, loc)) = parse_preposition(tokenizer) {
        todo!()
    }
    todo!()
}