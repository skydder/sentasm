use crate::{Label, PrepositionPhrases, Verb};
use tokenizer::Location;

#[derive(Debug)]
pub struct Sentence<'a> {
    pub verb: Verb<'a>,
    pub location: Location<'a>,
    pub preposition_phrases: PrepositionPhrases<'a>,
}

impl<'a> Sentence<'a> {
    pub fn new(
        verb: Verb<'a>,
        location: Location<'a>,
        prep_phrases: PrepositionPhrases<'a>,
    ) -> Self {
        Self {
            verb: verb,
            location: location,
            preposition_phrases: prep_phrases,
        }
    }
}

#[derive(Debug)]
pub enum Code<'a> {
    Sentence(Sentence<'a>),
    LabelDef(Label<'a>),
    Section(Label<'a>),
    NullStmt,
    RawNasm(&'a str),
}
