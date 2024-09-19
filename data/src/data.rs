use crate::emit_error_msg;

use super::{Loc, Result};
use super::{REG8, REG16, REG32, REG64, KEYWORD, VERB, PSEUDO, PREPOSITION};

// second parameter represents its size, and the third represents its value, which later use in mod-rm part.
// the forth represents wheather reg is r8~r15.
#[derive(Clone, Copy)]
pub struct Register<'a>(pub &'a str, pub usize, pub u8, pub bool);

// todo: add other register
impl<'a> Register<'a> {
    fn parse(token: &'a str) -> Option<Self> {
        for (i, reg) in REG8.into_iter().enumerate() {
            if token == *reg {
                let val = i as u8;
                return Some(Self(token, 8, val & 7, i >= 7));
            }
        }
        for (i, reg) in REG16.into_iter().enumerate() {
            if token == *reg {
                let val = i as u8;
                return Some(Self(token, 16, val & 7, i >= 7));
            }
        }
        for (i, reg) in REG32.into_iter().enumerate() {
            if token == *reg {
                let val = i as u8;
                return Some(Self(token, 32, val & 7, i >= 7));
            }
        }
        for (i, reg) in REG64.into_iter().enumerate() {
            if token == *reg {
                let val = i as u8;
                return Some(Self(token, 64, val & 7, i >= 7));
            }
        }
        None
    }
    pub fn is_reg(token: &'a str) -> bool {
        Self::parse(token).is_some()
    }
    pub fn is_64(&self) -> bool {
        self.3
    }
    pub fn reg(&self) -> u8 {
        self.2
    }
    pub fn size(&self) -> usize {
        self.1
    }
}

impl<'a> std::fmt::Debug for Register<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct DataSet<'a> {
    pub data: Data<'a>,
    pub loc: Loc<'a>,
}

#[derive(Clone, Copy)]
pub struct Immediate(pub u64, pub usize, pub bool);

impl Immediate {
    fn size_signed(i: i64) -> usize {
        if i > i8::MIN.into() && i < i8::MAX.into() {
            8
        } else if i > i16::MIN.into() && i < i16::MAX.into() {
            16
        } else if i > i32::MIN.into() && i < i32::MAX.into() {
            32
        } else if i > i64::MIN.into() && i < i64::MAX.into() {
            64
        } else {
            todo!()
        }
    }
    fn size(i: u64) -> usize {
        if i > u8::MIN.into() && i < u8::MAX.into() {
            8
        } else if i > u16::MIN.into() && i < u16::MAX.into() {
            16
        } else if i > u32::MIN.into() && i < u32::MAX.into() {
            32
        } else if i > u64::MIN.into() && i < u64::MAX.into() {
            64
        } else {
            todo!()
        }
    }
}

pub enum Data<'a> {
    Verb(Verb<'a>),
    Register(Register<'a>),
    Prepositon(Preposition<'a>),
    Immediate(Immediate),
    _Memory(&'a str),
    Memory(Memory<'a>),
    Label(Label<'a>),
    LabelDef,
    Section,
    Keyword(Keyword<'a>),
    _Define(&'a str),
    Define(Define<'a>),
}

impl<'a> DataSet<'a> {
    pub fn new(token: &'a str, loc: Loc<'a>) -> Self {
        Self {
            data: Data::parse(token),
            loc,
        }
    }
    pub fn expect_object(self) -> Option<Self> {
        match self.data {
            Data::Immediate(_)
            | Data::Keyword(_)
            | Data::Label(_)
            | Data::Register(_)
            | Data::Memory(_)
            | Data::Define(_) => Some(self),
            Data::_Memory(mem) => {
                let m = if let Some(memory) = Memory::new().parse(mem).ok() {
                    memory
                } else {
                    return None;
                };
                Some(Self {
                    data: Data::Memory(m),
                    loc: self.loc,
                })
            }
            Data::_Define(def) => {
                let d = if let Some(defn) = Define::new().parse(def).ok() {
                    defn
                } else {
                    return None;
                };
                Some(Self {
                    data: Data::Define(d),
                    loc: self.loc,
                })
            }
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
            Data::_Define(def) => {
                let d = if let Some(defn) = Define::new().parse(def).ok() {
                    defn
                } else {
                    return None;
                };
                Some(Self {
                    data: Data::Define(d),
                    loc: self.loc,
                })
            }
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
    pub fn get_register(&self) -> Option<Register> {
        match self.data {
            Data::Register(reg) => Some(reg),
            _ => None
        }
    }

    pub fn size(&self) -> usize{
        match &self.data {
            Data::Register(reg) => reg.size(),
            Data::Memory(mem) => mem.size(),
            Data::Label(_) => 32,
            Data::Immediate(_imm) => 8, // for now
            _ => 0
        }
    }
}

impl<'a> std::fmt::Debug for DataSet<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<'a> Data<'a> {
    pub fn parse(token: &'a str) -> Data {
        if token.starts_with("*(") {
            Data::_Memory(token)
        } else if token.starts_with("[") & token.ends_with("]") {
            Data::_Define(token)
        } else if token == "@" {
            Data::Section
        } else if token == "#" {
            Data::LabelDef
        } else if let Some(v) = Verb::parse(token) {
            Data::Verb(v)
        } else if let Some(r) = Register::parse(token) {
            Data::Register(r)
        } else if let Ok(i) = token.parse::<i64>() {
            if i < 0 {
                Data::Immediate(Immediate(i as u64, Immediate::size(i as u64), true))
            } else {
                Data::Immediate(Immediate(i as u64, Immediate::size(i as u64), false))
            }
        } else if let Some(k) = Keyword::parse(token) {
            Data::Keyword(k)
        } else if let Some(p) = Preposition::parse(token) {
            Data::Prepositon(p)
        } else {
            Data::Label(Label(token))
        }
    }
}

impl<'a> std::fmt::Debug for Data<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Verb(arg0) => write!(f, "{:?}", arg0),
            Self::Register(arg0) => write!(f, "{:?}", arg0),
            Self::Prepositon(arg0) => write!(f, "{:?}", arg0),
            Self::Immediate(arg0) => write!(f, "{}", arg0.0),
            Self::_Memory(arg0) => write!(f, "{:?}", arg0),
            Self::Memory(arg0) => write!(f, "{:?}", arg0),
            Self::Label(arg0) => write!(f, "{}", arg0.0),
            Self::LabelDef => write!(f, "LabelDef"),
            Self::Section => write!(f, "section"),
            Self::Keyword(arg0) => write!(f, "{:?}", arg0),
            Self::_Define(_) => todo!(),
            Self::Define(arg0) => write!(f, "{:?}", arg0),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Verb<'a>(pub &'a str);

impl<'a> Verb<'a> {
    fn parse(token: &'a str) -> Option<Self> {
        for verb in VERB {
            if token == *verb {
                return Some(Self(token));
            }
        }
        for verb in PSEUDO {
            if token == *verb {
                return Some(Self(token));
            }
        }
        None
    }
}
impl<'a> std::fmt::Debug for Verb<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self("add") => write!(f, "add"),
            Self("substract") => write!(f, "sub"),
            Self("multiply") => write!(f, "mul"),
            Self("divide") => write!(f, "div"),
            Self("move") => write!(f, "mov"),
            Self("jump") => write!(f, "jmp"),
            Self("and") => write!(f, "and"),
            Self("or") => write!(f, "or"),
            Self("xor") => write!(f, "xor"),
            Self("not") => write!(f, "not"),
            Self("negate") => write!(f, "neg"),
            Self("shift-rigt") => write!(f, "shr"),
            Self("shift-left") => write!(f, "shl"),
            Self("call") => write!(f, "call"),
            Self("compare") => write!(f, "cmp"),
            Self("return") => write!(f, "ret"),
            Self("leave") => write!(f, "leave"),
            Self("no-operation") => write!(f, "nop"),
            Self("systemcall") => write!(f, "syscall"),
            Self("halt") => write!(f, "hlt"),
            Self("load-effective-address") => write!(f, "lea"),
            Self("define") => write!(f, "data"),
            Self("push") => write!(f, "push"),
            Self("pop") => write!(f, "pop"),
            Self("globalize") => write!(f, "global"),
            Self("allocate") => write!(f, "bss"),
            Self("set-byte") => write!(f, "set"),
            Self("extend-*ax-reg") => write!(f, "cqo, etc"),
            Self("extern") => write!(f, "extern"),
            _ => write!(f, "Not Implemented"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Keyword<'a>(pub &'a str);

impl<'a> std::fmt::Debug for Keyword<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword("double-precision-float") => write!(f, "sd"),
            Keyword("single-precision-float") => write!(f, "ss"),
            Keyword("==") => write!(f, "e"),
            Keyword(">") => write!(f, "g"),
            Keyword("<") => write!(f, "l"),
            Keyword("!=") => write!(f, "ne"),
            Keyword(">=") => write!(f, "ge"),
            Keyword("<=") => write!(f, "le"),
            Keyword("8bit") => write!(f, "b"),
            Keyword("16bit") => write!(f, "w"),
            Keyword("32bit") => write!(f, "d"),
            Keyword("64bit") => write!(f, "q"),
            Keyword("signed") => write!(f, ""),
            _ => write!(f, "**"),
        }
    }
}

impl<'a> Keyword<'a> {
    fn parse(token: &'a str) -> Option<Self> {
        for key in KEYWORD {
            if token == *key {
                return Some(Self(token));
            }
        }
        None
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Preposition<'a>(pub &'a str);

impl<'a> Preposition<'a> {
    fn parse(token: &'a str) -> Option<Self> {
        for prep in PREPOSITION {
            if token == *prep {
                return Some(Self(token));
            }
        }
        return None;
    }
}

#[derive(Clone, Copy)]
pub struct Label<'a>(pub &'a str);

// this should have segment register

pub struct Memory<'a> {
    pub base: Option<Register<'a>>,
    pub displacement: Option<Box<DataSet<'a>>>,
    pub index: Option<Register<'a>>,
    pub scale: Option<u8>,
    pub size: usize,
    pub disp_size: usize
}

impl<'a> Memory<'a> {
    pub fn new() -> Self {
        Self {
            base: None,
            displacement: None,
            index: None,
            scale: None,
            size: 0,
            disp_size: 0
        }
    }

    // base = reg
    // index = reg
    // scale = 1|2|4|8
    // disp = num
    // mem = '['((base ('+' index ('*' scale)?)? ('+' disp)? ]) | (index '*' scale '+')? disp]))
    pub fn parse(mut self, token: &'a str) -> Result<Self> {
        let token_seq = Self::tokenize(token);
        let mut pos = 1;
        self.parse_base(&token_seq, &mut pos)?;
        Ok(self)
    }

    fn parse_base(&mut self, token_seq: &Vec<&'a str>, pos: &mut usize) -> Result<()> {
        if Register::is_reg(token_seq[*pos]) {
            if token_seq[*pos + 1] == ")"
                || token_seq[*pos + 1] == "+"
                || token_seq[*pos + 1] == "-"
                || token_seq[*pos + 1] == "by"
            {
                self.base = Register::parse(token_seq[*pos]);
                *pos += 1;
            }
        }
        self.parse_idx_scl(token_seq, pos)
    }

    fn parse_idx_scl(&mut self, token_seq: &Vec<&'a str>, pos: &mut usize) -> Result<()> {
        if token_seq[*pos] == "+" {
            *pos += 1;
        }
        if Register::is_reg(token_seq[*pos]) {
            if token_seq[*pos + 1] == ")"
                || token_seq[*pos + 1] == "+"
                || token_seq[*pos + 1] == "-"
            {
                self.index = Register::parse(token_seq[*pos]);
                *pos += 1;
            } else if token_seq[*pos + 1] == "*" {
                self.index = Register::parse(token_seq[*pos]);
                *pos += 2;
                // todo: emit error data
                self.scale = Some(token_seq[*pos].parse::<u8>().or_else(|_| Err(()))?);
                *pos += 1;
            }
        }
        self.parse_disp(token_seq, pos)
    }

    fn parse_disp(&mut self, token_seq: &Vec<&'a str>, pos: &mut usize) -> Result<()> {
        let sgn: bool;
        if token_seq[*pos] == "+" {
            *pos += 1;
            sgn = false;
        } else if token_seq[*pos] == "-" {
            *pos += 1;
            sgn = true;
        } else if token_seq[*pos] == ")" {
            return Ok(());
        } else if token_seq[*pos] == "by" {
            return self.parse_size(token_seq, pos);
        } else {
            sgn = false
        }
        // todo: emit error data

        let data = DataSet::new(token_seq[*pos], Loc::new("", 0, 0));
        match data.data {
            Data::Immediate(i) => {
                self.displacement = Some(Box::new(DataSet {
                    data: Data::Immediate(Immediate(i.0, Immediate::size(i.0), sgn)),
                    loc: data.loc,
                }));
                self.disp_size = Immediate::size(i.0);
            }
            Data::Label(_) => {
                self.displacement = Some(Box::new(data));
                self.disp_size = 32; // for now
            },
            _ => {
                emit_error_msg!("unexpected grammar", data.loc);
                return Err(());
            }
        }

        *pos += 1;
        self.parse_size(token_seq, pos)
    }

    fn parse_size(&mut self, token_seq: &Vec<&str>, pos: &mut usize) -> Result<()> {
        if token_seq[*pos] == "by" {
            *pos += 1;
            match token_seq[*pos] {
                "8bit" => self.size = 8,
                "16bit" => self.size = 16,
                "32bit" => self.size = 32,
                "64bit" => self.size = 64,
                _ => return Err(()),
            };
            *pos += 1;
        } else {
            self.size = 0;
        }
        if token_seq[*pos] == ")" {
            Ok(())
        } else {
            Err(())
        }
    }

    fn tokenize(token: &'a str) -> Vec<&'a str> {
        let mut pos = 0;
        let mut list: Vec<&'a str> = Vec::new();
        while token.chars().nth(pos).is_some() {
            Self::skip_whitespase(token, &mut pos);
            let len = Self::length_of_symbol(token, pos);
            list.push(&token[pos..pos + len]);
            pos += len;
        }
        
        list
    }

    fn length_of_symbol(token: &'a str, pos: usize) -> usize {
        let mut len = 0;

        if &token[pos..pos + 1] == "+" {
            return 1;
        } else if token[pos..].starts_with("*(") {
            return 2;
        } else if &token[pos..pos + 1] == "-" {
            return 1;
        } else if &token[pos..pos + 1] == "*" {
            return 1;
        } else if &token[pos..pos + 1] == ")" {
            return 1;
        } 

        while !token.chars().nth(pos + len).unwrap_or('\n').is_whitespace()
            && token.chars().nth(pos + len).unwrap_or('\n') != '+'
            && token.chars().nth(pos + len).unwrap_or('\n') != '-'
            && token.chars().nth(pos + len).unwrap_or('\n') != ')'
            && token.chars().nth(pos + len).unwrap_or('\n') != '*'
        {
            len += 1;
        }
        len
    }

    fn skip_whitespase(token: &'a str, pos: &mut usize) {
        while token.chars().nth(*pos).unwrap_or('*').is_whitespace() {
            *pos += 1;
        }
    }

    pub fn check_size_of_reg(&self, size: usize) -> bool {
        if self.base.is_some() && self.base.unwrap().size() != size {
            return false;
        }
        if self.base.is_some() && self.index.unwrap().size() != size {
            return false;
        }
        true
    }
    pub fn size(&self) -> usize {
        self.size
    }
}

impl<'a> std::fmt::Debug for Memory<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.size != 0 {
            match self.size {
                8 => write!(f, "byte"),
                16 => write!(f, "word"),
                32 => write!(f, "dword"),
                64 => write!(f, "qword"),
                _ => write!(f, ""), // should be error ....
            }?;
        }
        write!(f, "[")?;
        let mut count = 0;
        if let Some(base) = self.base {
            write!(f, "{:?}", base)?;
            count += 1;
        }

        if let Some(idx) = self.index {
            if count > 0 {
                write!(f, "+")?;
            }
            write!(f, "{:?}", idx)?;
            if let Some(scl) = self.scale {
                write!(f, "*{}", scl)?;
            }
        }

        if let Some(disp) = &self.displacement {
            match **disp {
                DataSet {
                    data: Data::Immediate(i),
                    loc: _,
                } => {
                    if i.0 >= 0 && count > 0 {
                        write!(f, "+")?;
                    }
                    write!(f, "{}", i.0)?;
                }
                DataSet {
                    data: Data::Label(l),
                    loc: _,
                } => {
                    if count > 0 {
                        write!(f, "+")?;
                    }
                    write!(f, "{}", l.0)?;
                }
                _ => todo!(),
            }
        }
        write!(f, "]")
    }
}

// later to implement float
enum DefItem<'a> {
    Int(i64),
    Str(&'a str),
}

impl<'a> std::fmt::Debug for DefItem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DefItem::Int(i) => write!(f, "{}", i),
            DefItem::Str(s) => write!(f, "\"{}\"", s),
        }
    }
}

pub struct Define<'a> {
    list: Vec<DefItem<'a>>,
}

impl<'a> Define<'a> {
    // Define = [DefItem (, DefItem)*]
    // DefItem = number | "string"
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn parse(mut self, token: &'a str) -> Result<Self> {
        let list = Self::tokenize(&token[1..token.len() - 1]);
        for i in 0..list.len() {
            self.list.push(Self::parse_item(list[i])?);
        }
        Ok(self)
    }

    fn tokenize(token: &'a str) -> Vec<&'a str> {
        let mut pos = 0;
        let mut list: Vec<&'a str> = Vec::new();
        while token.chars().nth(pos).is_some() {
            Self::skip_whitespase(token, &mut pos);
            let len = Self::length_of_symbol(token, pos);
            list.push(&token[pos..pos + len]);
            pos += len;
        }
        list
    }
    // todo: treat '"' in "string"
    fn length_of_symbol(token: &'a str, pos: usize) -> usize {
        let mut len = 0;

        if token[pos..].starts_with("\"") {
            len += 1;
            while token.chars().nth(pos + len).unwrap_or('"') != '"' {
                len += 1;
            }
            return len + 1;
        }

        while !token.chars().nth(pos + len).unwrap_or('\n').is_whitespace()
            && token.chars().nth(pos + len).unwrap_or(',') != ','
        {
            len += 1;
        }
        len
    }

    fn skip_whitespase(token: &'a str, pos: &mut usize) {
        while token.chars().nth(*pos).unwrap_or('*').is_whitespace()
            || token.chars().nth(*pos).unwrap_or(',') == ','
        {
            *pos += 1;
        }
    }

    fn parse_item(token: &'a str) -> Result<DefItem> {
        if let Ok(i) = token.parse() {
            Ok(DefItem::Int(i))
        } else if token.starts_with('"') & token.ends_with('"') {
            Ok(DefItem::Str(&token[1..token.len() - 1]))
        } else {
            Err(())
        }
    }
}

impl<'a> std::fmt::Debug for Define<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pos = 1;
        write!(f, "{:?}", self.list[0])?;
        while pos < self.list.len() {
            write!(f, ", {:?}", self.list[pos])?;
            pos += 1;
        }
        Ok(())
    }
}
