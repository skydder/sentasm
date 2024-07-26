use std::collections::HashMap;

use super::{Register, Data, DataSet, Label, Loc, Memory, Preposition, Result, Tonkenizer, Verb};

impl<'a> Memory<'a> {
    // reg + reg * num + reg
    // (reg '+'|']')? (reg ('*' (1|2|4|8))? '+'|']')? (num ])?
    // base = reg
    // index = reg 
    // scale = 1|2|4|8 
    // disp = num
    // mem = (base (+ index (* scale)?)? (+ disp)?) | index * scale + disp
    fn parse(token: &'a str) -> Result<Self> {
        let mut token_seq = token.replace("@[", " @[ ").replace("]", " ] ")
                                    .replace("*", " * ").replace("+", " + ").replace("-", " - ").split(' ').map(Data::parse).collect::<Vec<Data>>();
        
        let base: Option<(super::data::Register, Loc<'a>)> = None;
        let idx: Option<(super::data::Register, Loc<'a>)> = None;
        let scl: Option<(usize, Loc<'a>)> = None;
        let disp :Option<(i64, Loc<'a>)> = None;
        Ok(Self { base: base, displacement: disp, index: idx, scale: scl })
    }
    fn parse_reg(token_seq:Vec<Data>) {
        
    }
    
    

    
    
}

pub(crate) struct PrepositionPhrases<'a> {
    data: HashMap<Preposition, DataSet<'a>>
}

impl<'a> PrepositionPhrases<'a> {
    fn parse(tokenizer: &'a  Tonkenizer<'a>) -> Result<Self> {
        let mut data: HashMap<Preposition, DataSet<'a>> = HashMap::new(); 
        while let Some(DataSet { data:Data::Prepositon(p), loc: _ }) = tokenizer.next() {
            data.insert(p, tokenizer.peek().ok_or_else(|| eprintln!("preposition must take an object, but found nothing"))?.expect_object()?);
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

pub enum Code<'a> {
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
    pub fn parse(tonkenizer: &'a Tonkenizer<'a>) -> Result<Self> {
        match tonkenizer.next() {
            Some(DataSet { data: Data::Verb(v), loc }) => {
                let ret = Ok(Self::Sentence { verb: v, verb_loc: loc, object: tonkenizer.peek().ok_or_else(|| eprintln!("preposition must take an object, but found nothing"))?.expect_object().ok(), preposition_phrases:PrepositionPhrases::parse(tonkenizer )?});
                tonkenizer.next();
                ret
            },
            Some(DataSet { data: Data::Label(l), loc }) => Ok(Self::LabelDef(l)),
            None => Ok(Self::NullStmt),
            _ => todo!(),
        }
    }
}