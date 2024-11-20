use crate::DataSet;
use std::{cell::RefCell, collections::HashMap};
use tokenizer::{emit_error, Location};
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Preposition<'a>(pub &'a str);

impl<'a> std::fmt::Display for Preposition<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct PrepositionObject<'a> {
    pub object: DataSet<'a>,
    pub location: Location<'a>,
}

impl<'a> PrepositionObject<'a> {
    pub fn new(object: DataSet<'a>, location: Location<'a>) -> Self {
        if let Some(obj) = object.expect_object() {
            Self {
                object: obj,
                location: location,
            }
        } else {
            // error
            emit_error!(location, "expected object, but found other");
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
    pub fn new(data: RefCell<HashMap<Preposition<'a>, PrepositionObject<'a>>>) -> Self {
        Self { data: data }
    }
    pub fn get_object(&self, p: Preposition<'a>) -> Option<(DataSet, Location)> {
        self.data
            .borrow_mut()
            .remove(&p)
            .and_then(|po| Some(po.expand()))
    }

    pub fn len(&self) -> usize {
        self.data.borrow().len()
    }
}
