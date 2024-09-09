use std::{cell::RefCell, collections::HashMap};

use crate::emit_error_msg;

use super::{Data, DataSet, Label, Loc, Preposition, Result, Tonkenizer, Verb};

#[derive(Debug)]
pub(crate) struct PrepositionPhrases<'a> {
    data: RefCell<HashMap<Preposition<'a>, DataSet<'a>>>,
}

impl<'a> PrepositionPhrases<'a> {
    fn parse(tokenizer: &'a Tonkenizer<'a>) -> Result<Self> {
        let mut data: HashMap<Preposition, DataSet<'a>> = HashMap::new();
        while let Some(DataSet {
            data: Data::Prepositon(p),
            loc,
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
    pub(crate) fn get_object(&self, p: Preposition<'a>) -> Option<DataSet> {
        self.data.borrow_mut().remove(&p)
    }
}

pub struct Sentence <'a> {
    pub verb: Verb<'a>,
    pub verb_loc: Loc<'a>,
    pub object: Option<DataSet<'a>>,
    pub preposition_phrases: PrepositionPhrases<'a>,
}
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
            }) => {
                let object = if let Some(data) = tonkenizer.peek() {
                    data.expect_object()
                } else {
                    None
                };
                if object.is_some() {
                    tonkenizer.next();
                }
                let ret = Ok(Self::Sentence(Sentence{
                    verb: v,
                    verb_loc: loc,
                    object,
                    preposition_phrases: PrepositionPhrases::parse(tonkenizer)?,
                }));

                ret
            }
            Some(DataSet {
                data: Data::LabelDef,
                loc,
            }) => {
                if let Some(DataSet {
                    data: Data::Label(l),
                    loc: _,
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
            }) => {
                if let Some(DataSet {
                    data: Data::Label(l),
                    loc: _,
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