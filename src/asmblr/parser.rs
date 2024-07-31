use std::collections::HashMap;

use super::{Data, DataSet, Label, Loc, Preposition, Result, Tonkenizer, Verb};

#[derive(Debug)]
pub(crate) struct PrepositionPhrases<'a> {
    data: HashMap<Preposition, DataSet<'a>>,
}

impl<'a> PrepositionPhrases<'a> {
    fn parse(tokenizer: &'a Tonkenizer<'a>) -> Result<Self> {
        let mut data: HashMap<Preposition, DataSet<'a>> = HashMap::new();
        while let Some(DataSet {
            data: Data::Prepositon(p),
            loc: _,
        }) = tokenizer.next()
        {
            data.insert(
                p,
                tokenizer
                    .peek()
                    .ok_or_else(|| eprintln!("preposition must take an object, but found nothing"))?
                    .expect_object()?,
            );
            tokenizer.next();
        }
        Ok(Self { data })
    }
    pub(crate) fn expect_empty(&self) -> Result<()> {
        if self.data.is_empty() {
            Ok(())
        } else {
            eprintln!("expected no more preposition phrases, but found it");
            Err(())
        }
    }
    pub(crate) fn get_object(&mut self, p: Preposition) -> Option<DataSet> {
        self.data.remove(&p)
    }
}

#[derive(Debug)]
pub enum Code<'a> {
    Sentence {
        verb: Verb,
        verb_loc: Loc<'a>,
        object: Option<DataSet<'a>>,
        preposition_phrases: PrepositionPhrases<'a>,
    },
    LabelDef(Label<'a>),
    NullStmt,
}

impl<'a> Code<'a> {
    pub fn parse(tonkenizer: &'a Tonkenizer<'a>) -> Result<Self> {
        match tonkenizer.next() {
            Some(DataSet {
                data: Data::Verb(v),
                loc,
            }) => {
                let ret = Ok(Self::Sentence {
                    verb: v,
                    verb_loc: loc,
                    object: tonkenizer
                        .peek()
                        .ok_or_else(|| {
                            eprintln!("preposition must take an object, but found nothing")
                        })?
                        .expect_object()
                        .ok(),
                    preposition_phrases: PrepositionPhrases::parse(tonkenizer)?,
                });
                tonkenizer.next();
                ret
            }
            Some(DataSet {
                data: Data::Label(l),
                loc,
            }) => Ok(Self::LabelDef(l)),
            None => Ok(Self::NullStmt),
            _ => todo!(),
        }
    }
}
