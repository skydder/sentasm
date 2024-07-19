use std::collections::HashMap;

use super::{Data, DataSet, Label, Loc, Preposition, Tonkenizer, Verb, Result};
struct PrepositionPhrases<'a> {
    data: HashMap<Preposition, DataSet<'a>>
}

impl<'a> PrepositionPhrases<'a> {
    fn parse(tokenizer: &'a  Tonkenizer<'a>) -> Result<Self> {
        let mut data: HashMap<Preposition, DataSet<'a>> = HashMap::new(); 
        while let Some(DataSet { data:Data::Prepositon(p), loc: _ }) = tokenizer.next_dataset() {
            data.insert(p, tokenizer.next_dataset().ok_or_else(|| eprintln!("preposition must take an object, but found nothing"))?);
        }
        Ok(Self { data })
    }
}

pub(crate) enum Code<'a> {
    Sentence {
        verb: Verb,
        verb_loc: Loc<'a>,
        object: Option<DataSet<'a>>,
        preposition_phrases: PrepositionPhrases<'a>
    },
    LabelDef(Label<'a>),
    NullStmt,
}

impl<'a> Code<'a> {
    fn parse(tonkenizer: &'a Tonkenizer<'a>) -> Result<Self> {
        match tonkenizer.next_dataset() {
            Some(DataSet { data: Data::Verb(v), loc }) => {
                Ok(Self::Sentence { verb: v, verb_loc: loc, object: tonkenizer.next_dataset(), preposition_phrases:PrepositionPhrases::parse(tonkenizer )?})
            },
            Some(DataSet { data: Data::Label(l), loc }) => todo!(),
            None => Ok(Self::NullStmt),
            _ => todo!(),
        }
    }
}