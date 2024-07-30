use std::collections::HashMap;

use super::{Register, Data, DataSet, Label, Loc, Memory, Preposition, Result, Tonkenizer, Verb};

impl Memory {
    pub fn new() -> Self {
        Self { base: None, displacement: None, index: None, scale: None }
    }

    // reg + reg * num + reg
    // (reg '+'|']')? (reg ('*' (1|2|4|8))? '+'|']')? (num ])?
    // base = reg
    // index = reg 
    // scale = 1|2|4|8 
    // disp = num
    // mem = (base (+ index (* scale)?)? (+ disp)?) | (index * scale +)? disp
    pub fn parse<'a>(&mut self, token: &'a str) -> Result<()> {
        let tokens = token.replace("@[", "[ ").replace("]", " ]")
                                    .replace("*", " * ").replace("+", " + ").replace("-", " - ");
        let token_seq = tokens.split_ascii_whitespace().map(Data::parse).collect::<Vec<Data>>();


        println!("===========mem========");
        for i in tokens.split(' ').map(Data::parse).collect::<Vec<Data>>() {
            println!("{:?}", i);
        }

        let mut pos = 1;
        self.parse_base(&token_seq, &mut pos);
        self.parse_idx_scl(&token_seq, &mut pos)?;
        self.parse_disp(&token_seq, &mut pos)?;
        println!("===========mem========");
        Ok(())
    }
    fn parse_base(&mut self, token_seq: &Vec<Data>, pos:&mut usize) {
        println!("{}", *pos);
        println!("base:{:?}", token_seq[*pos]);
        self.base = match token_seq[*pos] {
            Data::Register(reg) => {
                match token_seq[*pos + 1] {
                    Data::Label("]") | Data::Label("+") | Data::Label("-") => {
                        *pos += 1;
                         Some(reg)
                    },
                    _ => None
                }
            },
            _ => None,
        };
    }

    fn parse_idx_scl(&mut self, token_seq: &Vec<Data>, pos:&mut usize) -> Result<()> {
        if let Data::Label("+") = token_seq[*pos] {
            *pos +=1;
        }
        if *pos >= token_seq.len() {
            return Ok(());
        }
        (self.index, self.scale) = match token_seq[*pos] {
            Data::Register(reg) => {
                *pos += 1;
                match (token_seq[*pos], token_seq[*pos + 1]) {
                    (Data::Label("*"), Data::Immediate(imm)) => {
                        *pos += 2;
                        (Some(reg), Some(TryInto::<usize>::try_into(imm).or_else(|_| Err(()))?))
                    },
                    (Data::Label("]") | Data::Label("+") | Data::Label("-"), _) => {
                        if self.base.is_none() {
                            return Err(());
                        }
                        (Some(reg), None)
                    },
                    // (Data::Label(""), _) => {
                    //     *pos += 1;
                    //     self.parse_idx_scl(token_seq, pos)?;
                    //     (self.index, self.scale)
                    // }
                    _ => {
                        eprintln!("unexpected");
                        return Err(());
                    }
                }
            },
            _ => (None, None),
        };

        Ok(())
    }

    fn parse_disp(&mut self, token_seq: &Vec<Data>, pos:&mut usize) -> Result<()> {
        if let Data::Label("+") = token_seq[*pos] {
            *pos +=1;
        }
        if *pos >= token_seq.len() {
            return Ok(());
        }
        println!("{}", *pos);
        println!("disp:{:?}", token_seq[*pos]);
        self.displacement = match token_seq[*pos] {
            Data::Immediate(imm) => {
                Some(imm)
            },
            Data::Label("-") => {
                *pos += 1;
                match token_seq[*pos] {
                    Data::Immediate(imm) => {
                        *pos += 1;
                        Some(-imm)
                    }
                    _ => {
                        eprintln!("expected immediate, but found other1");
                        return Err(());
                    }
                }
            }
            Data::Label("]") => None,
            _ => {
                eprintln!("expected immediate, but found other");
                return Err(());
            }
        };
        Ok(())
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