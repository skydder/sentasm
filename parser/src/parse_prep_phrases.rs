use std::{cell::RefCell, collections::HashMap};

use data::{Data, DataSet, Preposition, PrepositionObject, PrepositionPhrases};
use macros::match_data2;
use tokenizer::{emit_error, Tokenizer};

use crate::parse_data_set;

pub(crate) fn parse_prep_phrases<'a>(
    tokenizer: &'a Tokenizer<'a>,
    mut data: HashMap<Preposition<'a>, PrepositionObject<'a>>,
) -> PrepositionPhrases<'a> {
    while let Some(match_data2!(Preposition, prep, location)) = parse_data_set(tokenizer) {
        if let Some(obj) = parse_data_set(tokenizer) {
            data.insert(prep, PrepositionObject::new(obj, location));
        } else {
            // error
            emit_error!(
                location,
                "expected object after this preposition, but could not find it"
            );
        }
    }
    PrepositionPhrases::new(RefCell::new(data))
}
