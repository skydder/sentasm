use std::{cell::RefCell, collections::HashMap};

use tokenizer::Location;

use crate::{emit_error_msg, SI};

use super::{Data, DataSet, Label, Loc, Preposition, Result, Tonkenizer, Verb};

#[derive(Debug)]
pub struct PrepositionObject<'a> {
    object: DataSet<'a>,
    location: Location<'a>
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
            todo!()
        }
    }
}

#[derive(Debug)]
pub struct PrepositionPhrases_<'a> {
    data: RefCell<HashMap<Preposition<'a>, PrepositionObject<'a>>>,
}

#[derive(Debug)]
pub struct PrepositionPhrases<'a> {
    data: RefCell<HashMap<Preposition<'a>, DataSet<'a>>>,
}

impl<'a> PrepositionPhrases_<'a> {
    pub fn new(data: RefCell<HashMap<Preposition<'a>, PrepositionObject<'a>>>)  -> Self {
        Self { data: data }
    }
}

impl<'a> PrepositionPhrases<'a> {
    fn parse(tokenizer: &'a Tonkenizer<'a>, mut data:HashMap<Preposition<'a>, DataSet<'a>>) -> Result<Self> {
        // let mut data: HashMap<Preposition, DataSet<'a>> = HashMap::new();
        while let Some(DataSet {
            data: Data::Preposition(p),
            loc,
            location: _
        }) = tokenizer.next()
        {
            data.insert(
                p,
                tokenizer
                    .peek()
                    .map_or_else(|| None, |date| date.expect_object())
                    .ok_or_else(|| {
                        emit_error_msg!("preposition must take an object, but found nothing.", loc);
                    })?,
            );
            tokenizer.next();
        }
        Ok(Self {
            data: RefCell::new(data),

        })
    }
    pub fn get_object(&self, p: Preposition<'a>) -> Option<DataSet> {
        self.data.borrow_mut().remove(&p)
    }
}

#[derive(Debug)]
pub struct Sentence <'a> {
    pub verb: Verb<'a>,
    pub verb_loc: Loc<'a>,
    pub location: Location<'a>,
    pub preposition_phrases: PrepositionPhrases<'a>,
    preposition_phrases_: PrepositionPhrases_<'a>
}

impl<'a> Sentence<'a> {
    pub fn new(verb: Verb<'a>, location: Location<'a>, prep_phrases: PrepositionPhrases_<'a>) -> Self {
        let prep = PrepositionPhrases {
            data: RefCell::new(HashMap::new())
        };
        Self { verb: verb, verb_loc: Loc::new("", 0, 0), location: location, preposition_phrases: prep, preposition_phrases_: prep_phrases }
    }
}

#[derive(Debug)]
pub enum Code<'a> {
    Sentence(Sentence<'a>),
    LabelDef(Label<'a>),
    Section(Label<'a>),
    NullStmt,
}

impl<'a> Code<'a> {
    pub fn parse(tonkenizer: &'a Tonkenizer<'a>) -> Result<Self> {
        match tonkenizer.next() {
            Some(DataSet {
                data: Data::Verb(v),
                loc,
                location: _
            }) => {
                let mut preps: HashMap<Preposition, DataSet<'a>> = HashMap::new();
                let object = if let Some(data) = tonkenizer.peek() {
                    data.expect_object()
                } else {
                    None
                };
                if object.is_some() {
                    tonkenizer.next();
                    preps.insert(Preposition("obj"), object.unwrap());
                }
                let ret = Ok(Self::Sentence(Sentence{
                    verb: v,
                    verb_loc: loc,
                    preposition_phrases: PrepositionPhrases::parse(tonkenizer, preps)?,
                    preposition_phrases_: PrepositionPhrases_ { data: RefCell::new(HashMap::new()) },
                    location: Location::new(&SI)
                }));

                ret
            }
            Some(DataSet {
                data: Data::LabelDef,
                loc,
                location: _
            }) => {
                if let Some(DataSet {
                    data: Data::Label(l),
                    loc: _,
                    location: _
                }) = tonkenizer.peek()
                {
                    tonkenizer.next();
                    Ok(Self::LabelDef(l))
                } else {
                    emit_error_msg!(
                        "expected label definition, but this is not the label definition.",
                        loc
                    );
                    Err(())
                }
            }
            Some(DataSet {
                data: Data::Section,
                loc,
                location: _
            }) => {
                if let Some(DataSet {
                    data: Data::Label(l),
                    loc: _,
                    location: _
                }) = tonkenizer.peek()
                {
                    tonkenizer.next();
                    Ok(Self::Section(l))
                } else {
                    emit_error_msg!(
                        "expected label definition, but this is not the label definition.",
                        loc
                    );
                    Err(())
                }
            }
            None => Ok(Self::NullStmt),
            _ => {
                emit_error_msg!(format!("unexpected token:{:?}", tonkenizer), tonkenizer.loc());
                Err(())
            }
        }
    }
}