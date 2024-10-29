use tokenizer::Location;
use crate::{Memory, Verb, Label, Define, Register, Preposition, Immediate, Keyword};

#[derive(Debug)]
pub struct DataSet<'a> {
    pub data: Data<'a>,
    pub location: Location<'a>
}

#[derive(Debug)]
pub enum Data<'a> {
    Verb(Verb<'a>),
    Register(Register<'a>),
    Preposition(Preposition<'a>),
    Immediate(Immediate),
    Memory(Memory<'a>),
    Label(Label<'a>),
    LabelDef,
    Section,
    Keyword(Keyword<'a>),
    Define(Define<'a>)
}

impl<'a> DataSet<'a> {

    pub fn new(data: Data<'a>, location: Location<'a>) -> Self {
        Self { data, location }
    }
    pub fn expect_object(self) -> Option<Self> {
        match self.data {
            Data::Immediate(_)
            | Data::Keyword(_)
            | Data::Label(_)
            | Data::Register(_)
            | Data::Memory(_)
            | Data::Define(_) => Some(self),
            _ => None,
        }
    }

    pub fn expect_label(self) -> Option<Self> {
        match self.data {
            Data::Label(_) => Some(self),
            _ => None,
        }
    }

    pub fn expect_immediate(self) -> Option<Self> {
        match self.data {
            Data::Immediate(_) => Some(self),
            _ => None,
        }
    }

    pub fn expect_define(self) -> Option<Self> {
        match self.data {
            Data::Define(_) => Some(self),
            _ => None,
        }
    }

    pub fn expect_keyword(self) -> Option<Self> {
        match self.data {
            Data::Keyword(_) => Some(self),
            _ => None,
        }
    }

    pub fn is_register(&self) -> bool {
        match self.data {
            Data::Register(_) => true,
            _ => false
        }
    }
    pub fn get_register(self) -> Option<Register<'a>> {
        match self.data {
            Data::Register(reg) => Some(reg),
            _ => None
        }
    }

    pub fn get_immediate(self) -> Option<Immediate> {
        match self.data {
            Data::Immediate(imm) => Some(imm),
            _ => None
        }
    }

    pub fn get_memory(self) -> Option<Memory<'a>> {
        match self.data {
            Data::Memory(mem) => Some(mem),
            _ => None
        }
    }

    pub fn size(&self) -> usize{
        match &self.data {
            Data::Register(reg) => reg.size(),
            Data::Memory(mem) => mem.size(),
            Data::Label(_) => 32,
            Data::Immediate(_imm) => _imm.1,
            _ => 0
        }
    }
}

impl<'a> std::fmt::Display for DataSet<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}


impl<'a> std::fmt::Display for Data<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Verb(arg0) => write!(f, "{}", arg0),
            Self::Register(arg0) => write!(f, "{}", arg0),
            Self::Preposition(arg0) => write!(f, "{}", arg0),
            Self::Immediate(arg0) => write!(f, "{}", arg0.0),
            Self::Memory(arg0) => write!(f, "{}", arg0),
            Self::Label(arg0) => write!(f, "{}", arg0.0),
            Self::LabelDef => write!(f, "LabelDef"),
            Self::Section => write!(f, "section"),
            Self::Keyword(arg0) => write!(f, "{}", arg0),
            Self::Define(arg0) => write!(f, "{}", arg0),
        }
    }
}
