use tokenizer::Location;

#[derive(Clone, Copy, Debug)]
pub enum RegType {
    GP8,
    GP16,
    GP32,
    GP64,
    X87_80,
    MMX,
    XMM,
    YMM,
    SReg,
    CReg,
    DReg,
}

// second parameter represents its size, and the third represents its value, which later use in mod-rm part.
// the forth represents wheather reg is r8~r15.
#[derive(Clone, Copy, Debug)]
pub struct Register<'a>(pub &'a str, pub usize, pub u8, pub bool, pub RegType);

// todo: add other register
impl<'a> Register<'a> {
    pub fn is_reg(token: &'a str) -> bool {
        use crate::data_auto::{REG8, REG16, REG32, REG64};
        macro_rules! contains {
            ($REG:expr, $token:expr) => {
                $REG.iter().find_map(|reg| if *reg == $token {
                    Some(())
                } else {
                    None
                }).is_some()
            };
        }
        contains!(REG8, token) || contains!(REG16, token) || contains!(REG32, token) || contains!(REG64, token)
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

impl<'a> std::fmt::Display for Register<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct DataSet<'a> {
    pub data: Data<'a>,
    pub location: Location<'a>
}

macro_rules! split_bytes {
    ($bytes: expr, $nth: expr, $type: ty) => {
        (($bytes >> $nth) as $type, ($bytes - (($bytes >> $nth) << $nth)) as $type)
    };
}

#[derive(Clone, Copy, Debug)]
pub struct Immediate(pub u64, pub usize, pub bool);

impl Immediate {
    #[allow(warnings)]
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
    pub fn size(i: u64) -> usize {
        if i < u8::MAX.into() {
            8
        } else if i < u16::MAX.into() {
            16
        } else if i < u32::MAX.into() {
            32
        } else if i < u64::MAX.into() {
            64
        } else {
            todo!()
        }
    }
    pub fn generate(self) -> Vec<u8> {
        match self.1 {
            8 => {
                let byte = if self.2 {
                    u8::MAX - (self.0 as u8 - 1)
                } else {
                    self.0 as u8
                };
                return vec![byte];
            },
            16 => {
                let byte = if self.2 {
                    u16::MAX - (self.0 as u16 - 1)
                } else {
                    self.0 as u16
                };
                let (high, low) =  split_bytes!(byte, 8, u8);
                return vec![low, high];
            },
            32 => {
                let byte = if self.2 {
                    u32::MAX - (self.0 as u32 - 1)
                } else {
                    self.0 as u32
                };
                let (high, low) = split_bytes!(byte, 16, u16);
                let (hh, hl) = split_bytes!(high, 8, u8);
                let (lh, ll) = split_bytes!(low, 8, u8);
                
                return vec![ll, lh, hl, hh];
            },
            64 => {
                let byte = if self.2 {
                    u64::MAX - (self.0 as u64 - 1)
                } else {
                    self.0 as u64
                };

                let (high, low) = split_bytes!(byte, 32, u32);
                let (hh, hl) = split_bytes!(high, 16, u16);
                let (lh, ll) = split_bytes!(low, 16, u16);
                let (hhh, hhl) = split_bytes!(hh, 8, u8);
                let (hlh, hll) = split_bytes!(hl, 8, u8);
                let (lhh, lhl) = split_bytes!(lh, 8, u8);
                let (llh, lll) = split_bytes!(ll, 8, u8);
                
                return vec![lll, llh, lhl, lhh, hll, hlh, hhl, hhh];
            }
            _ => {
                todo!()
            }
        }
        
    }
}

#[test]
fn test() {
    for i in Immediate(0x3c, 64, false).generate(){
        println!("{:02x}", i)
    }
}

impl std::fmt::Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.2 {
            write!(f, "-{}", self.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
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
            Data::Immediate(_imm) => 8, // for now
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

#[derive(Clone, Copy, Debug)]
pub struct Verb<'a>(pub &'a str);

impl<'a> std::fmt::Display for Verb<'a> {
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

#[derive(Clone, Copy, Debug)]
pub struct Keyword<'a>(pub &'a str);

impl<'a> std::fmt::Display for Keyword<'a> {
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
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Preposition<'a>(pub &'a str);

impl<'a> std::fmt::Display for Preposition<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Label<'a>(pub &'a str);

// this should have segment register

#[derive(Debug)]
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

    pub fn set_base(&mut self, reg: Register<'a>) {
        self.base = Some(reg);
    }

    pub fn set_index(&mut self, reg: Register<'a>) {
        self.index = Some(reg);
    }

    pub fn set_scale(&mut self, scale: u8) {
        self.scale = Some(scale);
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn set_disp(&mut self, disp: DataSet<'a>) {
        self.displacement = Some(Box::new(disp));
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

impl<'a> std::fmt::Display for Memory<'a> {
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
            write!(f, "{}", base)?;
            count += 1;
        }

        if let Some(idx) = self.index {
            if count > 0 {
                write!(f, "+")?;
            }
            write!(f, "{}", idx)?;
            if let Some(scl) = self.scale {
                write!(f, "*{}", scl)?;
            }
        }

        if let Some(disp) = &self.displacement {
            match **disp {
                DataSet {
                    data: Data::Immediate(i),
                    location: _
                } => {
                    if count > 0 && !i.2{
                        write!(f, "+")?;
                    } else if i.2 {
                        write!(f, "-")?;
                    }
                    write!(f, "{}", i.0)?;
                }
                DataSet {
                    data: Data::Label(l),
                    location: _
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
pub enum DefItem<'a> {
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

#[derive(Debug)]
pub struct Define<'a> {
    list: Vec<DefItem<'a>>,
}

impl<'a> Define<'a> {
    pub fn new(list: Vec<DefItem<'a>>) -> Self {
        Self { list: list }
    }

}

impl<'a> std::fmt::Display for Define<'a> {
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
