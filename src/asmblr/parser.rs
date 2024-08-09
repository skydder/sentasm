use std::{
    cell::RefCell,
    collections::HashMap,
    // fs::File,
    // io::{BufRead, BufReader},
};

use super::{Data, DataSet, Label, Loc, Preposition, Result, Tonkenizer, Verb};

#[derive(Debug)]
pub(crate) struct PrepositionPhrases<'a> {
    data: RefCell<HashMap<Preposition, DataSet<'a>>>,
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
                match tokenizer.peek() {
                    Some(data) => data.expect_object().ok_or_else(|| {
                        eprintln!("preposition must take an object, but found nothing\n->{}",
                        loc
                    );
                    })?,
                    None => {
                        eprintln!("preposition must take an object, but found nothing\n->{}",
                        loc
                    );
                        return Err(());
                    }
                },
            );
            tokenizer.next();
        }
        Ok(Self {
            data: RefCell::new(data),
        })
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

// enum RawData {
//     Int(Vec<i64>),
//     Float(Vec<f64>),
//     Str(Vec<char>),
// }

// struct SectionSentense<'a> {
//     size: usize,
//     name: Label<'a>,
//     data: RawData,
// }

// impl<'a> SectionSentense<'a> {
//     pub(crate) fn parse(tonkenizer: &'a Tonkenizer<'a>) -> Result<Self> {
//         todo!()
//     }
// }

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
            }) => {
                if let Some(DataSet {
                    data: Data::LabelDef,
                    loc: _,
                }) = tonkenizer.peek()
                {
                    Ok(Self::LabelDef(l))
                } else {
                    eprintln!(
                        "expected label definition, but this is not the label definition.\n->{}",
                        loc
                    );
                    Err(())
                }
            }
            None => Ok(Self::NullStmt),
            _ => todo!(),
        }
    }
}

// struct _Section<'a> {
//     data: RefCell<HashMap<Label<'a>, Vec<Code<'a>>>>,
// }

// pub struct Section<'a> {
//     current_section: Label<'a>,
//     file: &'a str,
//     section:_Section<'a>
// }

// impl<'a> Section<'a> {
//     fn new(cur_lbl: Label<'a>, file: &'a str) -> Self {
//         Self {
//             current_section: cur_lbl,
//             file: file,
//             section: _Section::new()
//         }
//     }

//     fn build(mut self) -> Result<()> {
//         let code = BufReader::new(File::open(self.file).unwrap())
//             .lines()
//             .collect::<Vec<_>>();
//         let mut ln = 0;
//         for line_result in code {
//             let line: String = line_result.unwrap().clone();
//             let _tokenizer = Tonkenizer::new(&line, Loc::new(self.file, ln, 0));
//             if let Some(x) = self.section.data.borrow_mut().get_mut(self.current_section) {
//                 x.push(Code::parse(&_tokenizer)?)
//             }
//             ln += 1;
//         }
//         Ok(())
//     }

//     fn parse_line(&mut self, tonkenizer: &'a Tonkenizer<'a>, section: &'a mut _Section<'a>) -> Result<Code<'a>> {
//         match tonkenizer.peek() {
//             Some(DataSet {
//                 data: Data::LabelSpecial,
//                 loc,
//             }) => {
//                 tonkenizer.next();
//                 match tonkenizer.peek() {
//                     Some(DataSet {
//                         data: Data::LabelDef,
//                         loc: _,
//                     }) => tonkenizer.next(),
//                     _ => todo!(),
//                 };
//                 let label = if let Some(DataSet {
//                     data: Data::Label(lbl),
//                     loc,
//                 }) = tonkenizer.peek()
//                 {
//                     tonkenizer.next();
//                     lbl
//                 } else {
//                     todo!()
//                 };

//                 self.current_section = label;
//                 if section.data.borrow_mut().get(label).is_none() {
//                     section.data.borrow_mut().insert(label, Vec::new());
//                 }
//                 Ok(Code::NullStmt)
//             }
//             _ => Ok(Code::parse(&tonkenizer)?),
//         }
//     }
// }

// impl<'a> _Section<'a> {
//     fn new() -> Self {
//         Self {
//             data: RefCell::new(HashMap::new()),
//         }
//     }
// }
