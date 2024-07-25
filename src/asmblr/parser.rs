use std::collections::HashMap;

use super::{Data, DataSet, Label, Loc, Preposition, Tonkenizer, Verb, Result, Memory};

impl<'a> Memory<'a> {
    fn parse(tokenizer:& Tonkenizer) -> Result<Self> {


        let base: Option<(super::data::Register, Loc<'a>)>;
        let idx: Option<(super::data::Register, Loc<'a>)>;
        let scl: Option<(usize, Loc<'a>)>;
        let disp :Option<(i64, Loc<'a>)>;

        match tokenizer.peek_next() {
            Some(DataSet { data:  Data::MemorySym("*"), loc }) => {
                (idx, scl) = Self::parse_idx_scl(tokenizer)?;
            },
            Some(DataSet { data:  Data::MemorySym("+"), loc }) |
            Some(DataSet { data:  Data::MemorySym("-"), loc }) |
            Some(DataSet { data:  Data::MemoryRP, loc }) => {
                match tokenizer.peek() {
                    Some(DataSet { data:  Data::Immediate(i), loc }) => {
                        disp = Some((i, loc));
                    },
                    Some(DataSet { data:  Data::Register(reg), loc }) => {
                        base = Some((reg, loc));
                    },
                    _ => return Err(());
                }
            },
            _ => todo!(),
        }  





        Ok(Self { base: base, displacement: disp, index: idx, scale: scl })
    }
    fn parse_disp(tokenizer:& Tonkenizer) -> Result<Option<(i64, Loc<'a>)>> {
        match tokenizer.peek() {
            Some(DataSet { data:  Data::Immediate(i), loc }) => {
                Ok(Some((i, loc)))
            }
            Some(DataSet { data:  Data::MemoryRP, loc }) => Ok(None),
            _ => todo!(),
        }
    }
    fn parse_base(tokenizer:& Tonkenizer) -> Result<Option<(Register, Loc<'a>)>> {
        match tokenizer.peek() {
            Some(DataSet { data:  Data::Register(reg), loc }) => {
                Ok(Some((reg, loc)))
            }
            Some(DataSet { data:  Data::MemoryRP, loc }) => Ok(None),
            _ => todo!(),
        }
    }
    fn parse_idx_scl(tokenizer:& Tonkenizer) -> Result<(Option<(Register, Loc<'a>)>, Option<(usize, Loc<'a>)>)> {
        if let Some(DataSet { data:  Data::MemoryRP, loc }) = tokenizer.peek() {
            return Ok((None, None));
        }
        match tokenizer.peek_next() {
            Some(DataSet { data:  Data::MemorySym("*"), loc }) => {
                let idx = tokenizer.next().unwrap_or(DataSet {data: Data::LabelSpecial, loc}).expect_register()?;
                tokenizer.next();
                let scl = tokenizer.next().unwrap_or(DataSet {data: Data::LabelSpecial, loc}).expect_immediate()?;
                Ok((Some((idx.data.reg()?, idx.loc)),Some((scl.data.imm()?.try_into().or_else(|_|  Err(()))?, scl.loc))))
            },
            Some(DataSet { data:  Data::MemorySym("+"), loc }) |
            Some(DataSet { data:  Data::MemorySym("-"), loc }) |
            Some(DataSet { data:  Data::MemoryRP, loc }) => {
                let idx = tokenizer.next().unwrap_or(DataSet {data: Data::LabelSpecial, loc}).expect_register()?;
                tokenizer.next();
                Ok((Some((idx.data.reg()?, idx.loc)),None)) 
            },
            _ => todo!(),
        }
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