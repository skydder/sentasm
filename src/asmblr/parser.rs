use std::collections::HashMap;
use std::cell::RefCell;

use super::{Data, DataSet, Label, Loc, Preposition, Result, Tonkenizer, Verb, codegen};

#[derive(Debug)]
pub(crate) struct PrepositionPhrases<'a> {
    data: RefCell<HashMap<Preposition, DataSet<'a>>>,
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
                match tokenizer.peek() {
                    Some(data) => data.expect_object().ok_or_else(|| eprintln!("preposition must take an object, but found nothing"))?,
                    None => {
                        eprintln!("preposition must take an object, but found nothing");
                        return Err(());
                    }
                });
            tokenizer.next();
        }
        Ok(Self { data:RefCell::new(data) })
    }
    pub(crate) fn expect_empty(&self) -> Result<()> {
        if self.data.borrow().is_empty() {
            Ok(())
        } else {
            eprintln!("expected no more preposition phrases, but found it");
            Err(())
        }
    }
    pub(crate) fn get_object(&self, p: Preposition) -> Option<DataSet> {
        self.data.borrow_mut().remove(&p)
    }
}


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
                let object = if let Some(data) = tonkenizer.peek() {
                   data.expect_object()
                } else {
                    None
                };
                if object.is_some() {
                    tonkenizer.next();
                }
                let ret = Ok(Self::Sentence {
                    verb: v,
                    verb_loc: loc,
                    object,
                    preposition_phrases: PrepositionPhrases::parse(tonkenizer)?,
                });
                
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

// impl<'a> std::fmt::Debug for Code<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", codegen(*self).ok().unwrap_or_else(|| {eprintln!("hwat?"); String::new()}))
//     }
// }
