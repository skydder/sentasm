use std::{cell::RefCell, collections::HashMap};

use tokenizer::Tokenizer;
use data::{PrepositionObject, PrepositionPhrases_, DataSet, Data, Preposition};
use macros::match_data2;

use crate::parse_data_set;

pub(crate) fn parse_prep_phrases<'a>(tokenizer: &'a Tokenizer<'a>, mut data: HashMap<Preposition<'a>, PrepositionObject<'a>>) -> PrepositionPhrases_<'a> {
    while let match_data2!(Preposition, prep, location) = parse_data_set(tokenizer) {
        data.insert(prep, PrepositionObject::new(parse_data_set(tokenizer), location));
    }
    PrepositionPhrases_::new(RefCell::new(data))
}
