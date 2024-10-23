use std::{cell::RefCell, collections::HashMap, process::exit};

use tokenizer::{emit_error, Location};

use super::{DataSet, Label, Preposition, Verb};

#[derive(Debug)]
pub struct PrepositionObject<'a> {
    pub object: DataSet<'a>,
    pub location: Location<'a>
}

impl<'a> PrepositionObject<'a> {
    pub fn new(object: DataSet<'a>, location: Location<'a>) -> Self {
        if let Some(obj) = object.expect_object() {
            Self {
                object: obj,
                location: location
            }
        } else {
            // error
            emit_error!(location, "expected object, but found other");
            exit(1);
        }
    } 
    fn expand(self) -> (DataSet<'a>, Location<'a>) {
        (self.object, self.location)
    }
}

#[derive(Debug)]
pub struct PrepositionPhrases<'a> {
    pub data: RefCell<HashMap<Preposition<'a>, PrepositionObject<'a>>>,
}

impl<'a> PrepositionPhrases<'a> {
    pub fn new(data: RefCell<HashMap<Preposition<'a>, PrepositionObject<'a>>>)  -> Self {
        Self { data: data }
    }
    pub fn get_object(&self, p: Preposition<'a>) -> Option<(DataSet, Location)> {
        self.data.borrow_mut().remove(&p).and_then(|po| Some(po.expand()))
    }
}

#[derive(Debug)]
pub struct Sentence <'a> {
    pub verb: Verb<'a>,
    pub location: Location<'a>,
    pub preposition_phrases: PrepositionPhrases<'a>,
}

impl<'a> Sentence<'a> {
    pub fn new(verb: Verb<'a>, location: Location<'a>, prep_phrases: PrepositionPhrases<'a>) -> Self {
        Self { verb: verb, location: location, preposition_phrases: prep_phrases }
    }
}

#[derive(Debug)]
pub enum Code<'a> {
    Sentence(Sentence<'a>),
    LabelDef(Label<'a>),
    Section(Label<'a>),
    NullStmt,
}