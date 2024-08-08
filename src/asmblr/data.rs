use core::str;
use std::fmt::Debug;

use super::{Loc, Result};

// use someday
const REG8: &[&'static str] = &[
    "al", "bl", "cl", "dl", "dil", "sil", "bpl", "spl", "r8b", "r9b", "r10b", "r11b", "r12b",
    "r13b", "r14b", "r15b",
];
const REG16: &[&'static str] = &[
    "ax", "bx", "cx", "dx", "di", "si", "bp", "sp", "r8w", "r9w", "r10w", "r11w", "r12w", "r13w",
    "r14w", "r15w",
];
const REG32: &[&'static str] = &[
    "eax", "ebx", "ecx", "edx", "edi", "esi", "ebp", "esp", "r8d", "r9d", "r10d", "r11d", "r12d",
    "r13d", "r14d", "r15d",
];
const REG64: &[&'static str] = &[
    "rax", "rbx", "rcx", "rdx", "rdi", "rsi", "rbp", "rsp", "r8", "r9", "r10", "r11", "r12", "r13",
    "r14", "r15",
];
const XMM: &[&'static str] = &[
    "xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7",
];
#[derive(Clone, Copy)]
pub(crate) struct Register {
    size: usize,
    kind: _Register,
}

impl Register {
    pub(crate) fn parse<'a>(token: &'a str) -> Option<Self> {
        let size = if let Some(s) = Self::size(token) {
            s
        } else {
            return None;
        };
        let kind = _Register::parse(token).unwrap();
        Some(Self { size, kind })
    }
    fn size<'a>(token: &'a str) -> Option<usize> {
        for i in REG8 {
            if token == *i {
                return Some(8);
            }
        }

        for i in REG16 {
            if token == *i {
                return Some(16);
            }
        }
        for i in REG32 {
            if token == *i {
                return Some(32);
            }
        }
        for i in REG64 {
            if token == *i {
                return Some(64);
            }
        }
        for i in XMM {
            if token == *i {
                return Some(128);
            }
        }
        None
    }
    pub fn is_reg(token: &str) -> bool {
        Self::parse(token).is_some()
    }
}

impl std::fmt::Debug for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

pub struct DataSet<'a> {
    pub data: Data<'a>,
    pub loc: Loc<'a>,
}

#[derive(Clone, Copy)]
pub enum Data<'a> {
    Verb(Verb),
    Register(Register),
    Prepositon(Preposition),
    Immediate(i64),
    _Memory(&'a str),
    Memory(Memory),
    Label(Label<'a>),
    LabelDef,
    LabelSpecial,
    Keyword(Keyword),
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
            Data::Immediate(_) | Data::Keyword(_) | Data::Label(_) | Data::Register(_) => {
                Some(self)
            }
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
            _ => None,
        }
    }

    pub fn size(&self) -> usize {
        match self.data {
            Data::Register(reg) => reg.size,
            Data::Immediate(imm) => {
                if (imm as u64) < (2 << 8) {
                    8
                } else if (imm as u64) < (2 << 16) {
                    16
                } else if (imm as u64) < (2 << 16) {
                    32
                } else {
                    64
                }
            }
            Data::Memory(mem) => mem.size,
            _ => 0,
        }
    }

    pub fn expect_register(self) -> Option<Self> {
        match self.data {
            Data::Register(_) => Some(self),
            _ => None,
        }
    }

    pub fn expect_label(self) -> Option<Self> {
        match self.data {
            Data::Label(_) => Some(self),
            _ => None,
        }
    }

    pub fn is_memory(&self) -> bool {
        match self.data {
            Data::_Memory(_) | Data::Memory(_) => true,
            _ => false,
        }
    }

    pub fn is_register(&self) -> bool {
        match self.data {
            Data::Register(_) => true,
            _ => false,
        }
    }
}

impl<'a> std::fmt::Debug for DataSet<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<'a> Data<'a> {
    pub(crate) fn parse(token: &'a str) -> Data {
        if token.starts_with("@[") {
            Data::_Memory(token)
        } else if token == "#" {
            Data::LabelSpecial
        } else if token == ":" {
            Data::LabelDef
        } else if let Some(v) = Verb::parse(token) {
            Data::Verb(v)
        } else if let Some(r) = Register::parse(token) {
            Data::Register(r)
        } else if let Ok(i) = token.parse::<i64>() {
            Data::Immediate(i)
        } else if let Some(k) = Keyword::parse(token) {
            Data::Keyword(k)
        } else if let Some(p) = Preposition::parse(token) {
            Data::Prepositon(p)
        } else {
            Data::Label(token)
        }
    }

    pub fn reg(self) -> Result<Register> {
        match self {
            Self::Register(reg) => Ok(reg),
            _ => {
                eprintln!("expected register, but found other token");
                Err(())
            }
        }
    }

    pub fn imm(self) -> Result<i64> {
        match self {
            Self::Immediate(imm) => Ok(imm),
            _ => {
                eprintln!("expected immediate, but found other token");
                Err(())
            }
        }
    }
    pub fn mem(self) -> Option<&'a str> {
        match self {
            Self::_Memory(mem) => Some(mem),
            _ => {
                eprintln!("expected memory, but found other token");
                None
            }
        }
    }
}

impl<'a> std::fmt::Debug for Data<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Verb(arg0) => write!(f, "{:?}", arg0),
            Self::Register(arg0) => write!(f, "{:?}", arg0),
            Self::Prepositon(arg0) => write!(f, "{:?}", arg0),
            Self::Immediate(arg0) => write!(f, "{}", arg0),
            Self::_Memory(arg0) => write!(f, "{:?}", arg0),
            Self::Memory(arg0) => write!(f, "{:?}", arg0),
            Self::Label(arg0) => write!(f, "{}", arg0),
            Self::LabelDef => write!(f, "LabelDef"),
            Self::LabelSpecial => write!(f, "LabelSpecial"),
            Self::Keyword(arg0) => write!(f, "{:?}", arg0),
        }
    }
}

#[derive(Clone, Copy)]

pub(crate) enum Verb {
    Add,
    Substract,
    Multiply,
    Divide,
    Move,
    Jump,
    And,
    Or,
    Xor,
    Not,
    Negate,
    ShiftRight,
    ShiftLeft,
    Call,
    Compare,
    LoadEffectiveAddress,
    // intransitive verbs
    Return,
    Leave,
    NoOperation,
    SystemCall,
    Halt,
    // pesudo instruction verb
    Define,
}

impl Verb {
    fn parse<'a>(token: &'a str) -> Option<Self> {
        match token {
            "add" => Some(Self::Add),
            "substract" => Some(Self::Substract),
            "multiply" => Some(Self::Multiply),
            "divide" => Some(Self::Divide),
            "move" => Some(Self::Move),
            "jump" => Some(Self::Jump),
            "and" => Some(Self::And),
            "or" => Some(Self::Or),
            "xor" => Some(Self::Xor),
            "not" => Some(Self::Not),
            "negate" => Some(Self::Negate),
            "shift-right" => Some(Self::ShiftRight),
            "shift-left" => Some(Self::ShiftLeft),
            "call" => Some(Verb::Call),
            "compare" => Some(Self::Compare),
            "load-effective-adress" => Some(Self::LoadEffectiveAddress),

            "return" => Some(Self::Return),
            "halt" => Some(Self::Halt),
            "leave" => Some(Self::Leave),
            "no-operation" => Some(Self::NoOperation),
            "systemcall" => Some(Self::SystemCall),
            "define" => Some(Self::Define),
            _ => None,
        }
    }
}

impl std::fmt::Debug for Verb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "add"),
            Self::Substract => write!(f, "sub"),
            Self::Multiply => write!(f, "mul"),
            Self::Divide => write!(f, "div"),
            Self::Move => write!(f, "mov"),
            Self::Jump => write!(f, "jmp"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Xor => write!(f, "xor"),
            Self::Not => write!(f, "not"),
            Self::Negate => write!(f, "neg"),
            Self::ShiftRight => write!(f, "shr"),
            Self::ShiftLeft => write!(f, "shl"),
            Self::Call => write!(f, "call"),
            Self::Compare => write!(f, "cmp"),
            Self::Return => write!(f, "ret"),
            Self::Leave => write!(f, "leave"),
            Self::NoOperation => write!(f, "nop"),
            Self::SystemCall => write!(f, "syscall"),
            Self::Halt => write!(f, "hlt"),
            Self::LoadEffectiveAddress => write!(f, "lea"),
            Self::Define => write!(f, "def"),
        }
    }
}

// temporary implementation
#[derive(Clone, Copy)]
enum _Register {
    // general purpose regiser
    AL,
    BL,
    CL,
    DL,
    DIL,
    SIL,
    BPL,
    SPL,
    R8B,
    R9B,
    R10B,
    R11B,
    R12B,
    R13B,
    R14B,
    R15B, // byte
    AX,
    BX,
    CX,
    DX,
    DI,
    SI,
    BP,
    SP,
    R8W,
    R9W,
    R10W,
    R11W,
    R12W,
    R13W,
    R14W,
    R15W, // word
    EAX,
    EBX,
    ECX,
    EDX,
    EDI,
    ESI,
    EBP,
    ESP,
    R8D,
    R9D,
    R10D,
    R11D,
    R12D,
    R13D,
    R14D,
    R15D, // doubleword
    RAX,
    RBX,
    RCX,
    RDX,
    RDI,
    RSI,
    RBP,
    RSP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15, //  quadword

    XMM0,
    XMM1,
    XMM2,
    XMM3,
    XMM4,
    XMM5,
    XMM6,
    XMM7,
}

impl _Register {
    pub(crate) fn parse<'a>(token: &'a str) -> Option<Self> {
        match token {
            "al" => Some(Self::AL),
            "bl" => Some(Self::BL),
            "cl" => Some(Self::CL),
            "dl" => Some(Self::DL),
            "dil" => Some(Self::DIL),
            "sil" => Some(Self::SIL),
            "bpl" => Some(Self::BPL),
            "spl" => Some(Self::SPL),
            "r8b" => Some(Self::R8B),
            "r9b" => Some(Self::R9B),
            "r10b" => Some(Self::R10B),
            "r11b" => Some(Self::R11B),
            "r12b" => Some(Self::R12B),
            "r13b" => Some(Self::R13B),
            "r14b" => Some(Self::R14B),
            "r15b" => Some(Self::R15B),
            "ax" => Some(Self::AX),
            "bx" => Some(Self::BX),
            "cx" => Some(Self::CX),
            "dx" => Some(Self::DX),
            "di" => Some(Self::DI),
            "si" => Some(Self::SI),
            "bp" => Some(Self::BP),
            "sp" => Some(Self::SP),
            "r8w" => Some(Self::R8W),
            "r9w" => Some(Self::R9W),
            "r10w" => Some(Self::R10W),
            "r11w" => Some(Self::R11W),
            "r12w" => Some(Self::R12W),
            "r13w" => Some(Self::R13W),
            "r14w" => Some(Self::R14W),
            "r15w" => Some(Self::R15W),
            "eax" => Some(Self::EAX),
            "ebx" => Some(Self::EBX),
            "ecx" => Some(Self::ECX),
            "edx" => Some(Self::EDX),
            "edi" => Some(Self::EDI),
            "esi" => Some(Self::ESI),
            "ebp" => Some(Self::EBP),
            "esp" => Some(Self::ESP),
            "r8d" => Some(Self::R8D),
            "r9d" => Some(Self::R9D),
            "r10d" => Some(Self::R10D),
            "r11d" => Some(Self::R11D),
            "r12d" => Some(Self::R12D),
            "r13d" => Some(Self::R13D),
            "r14d" => Some(Self::R14D),
            "r15d" => Some(Self::R15D),
            "rax" => Some(Self::RAX),
            "rbx" => Some(Self::RBX),
            "rcx" => Some(Self::RCX),
            "rdx" => Some(Self::RDX),
            "rdi" => Some(Self::RDI),
            "rsi" => Some(Self::RSI),
            "rbp" => Some(Self::RBP),
            "rsp" => Some(Self::RSP),
            "r8" => Some(Self::R8),
            "r9" => Some(Self::R9),
            "r10" => Some(Self::R10),
            "r11" => Some(Self::R11),
            "r12" => Some(Self::R12),
            "r13" => Some(Self::R13),
            "r14" => Some(Self::R14),
            "r15" => Some(Self::R15),
            "xmm0" => Some(Self::XMM0),
            "xmm1" => Some(Self::XMM1),
            "xmm2" => Some(Self::XMM2),
            "xmm3" => Some(Self::XMM3),
            "xmm4" => Some(Self::XMM4),
            "xmm5" => Some(Self::XMM5),
            "xmm6" => Some(Self::XMM6),
            "xmm7" => Some(Self::XMM7),
            _ => None,
        }
    }
}

impl std::fmt::Debug for _Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = match self {
            Self::AL => "al",
            Self::BL => "bl",
            Self::CL => "cl",
            Self::DL => "dl",
            Self::DIL => "dil",
            Self::SIL => "sil",
            Self::BPL => "bpl",
            Self::SPL => "spl",
            Self::R8B => "r8b",
            Self::R9B => "r9b",
            Self::R10B => "r10b",
            Self::R11B => "r11b",
            Self::R12B => "r12b",
            Self::R13B => "r13b",
            Self::R14B => "r14b",
            Self::R15B => "r15b",
            Self::AX => "ax",
            Self::BX => "bx",
            Self::CX => "cx",
            Self::DX => "dx",
            Self::DI => "di",
            Self::SI => "si",
            Self::BP => "bp",
            Self::SP => "sp",
            Self::R8W => "r8w",
            Self::R9W => "r9w",
            Self::R10W => "r10w",
            Self::R11W => "r11w",
            Self::R12W => "r12w",
            Self::R13W => "r13w",
            Self::R14W => "r14w",
            Self::R15W => "r15w",
            Self::EAX => "eax",
            Self::EBX => "ebx",
            Self::ECX => "ecx",
            Self::EDX => "edx",
            Self::EDI => "edi",
            Self::ESI => "esi",
            Self::EBP => "ebp",
            Self::ESP => "esp",
            Self::R8D => "r8d",
            Self::R9D => "r9d",
            Self::R10D => "r10d",
            Self::R11D => "r11d",
            Self::R12D => "r12d",
            Self::R13D => "r13d",
            Self::R14D => "r14d",
            Self::R15D => "r15d",
            Self::RAX => "rax",
            Self::RBX => "rbx",
            Self::RCX => "rcx",
            Self::RDX => "rdx",
            Self::RDI => "rdi",
            Self::RSI => "rsi",
            Self::RBP => "rbp",
            Self::RSP => "rsp",
            Self::R8 => "r8",
            Self::R9 => "r9",
            Self::R10 => "r10",
            Self::R11 => "r11",
            Self::R12 => "r12",
            Self::R13 => "r13",
            Self::R14 => "r14",
            Self::R15 => "r15",
            Self::XMM0 => "xmm0",
            Self::XMM1 => "xmm1",
            Self::XMM2 => "xmm2",
            Self::XMM3 => "xmm3",
            Self::XMM4 => "xmm4",
            Self::XMM5 => "xmm5",
            Self::XMM6 => "xmm6",
            Self::XMM7 => "xmm7",
        };
        write!(f, "{}", reg)
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Keyword {
    DoublePrecisionFloat,
    SinglePrecisionFloat,
    Signed,
    ZeroExtened,
    E,
    G,
    L,
    NE,
    GE,
    LE,
}

impl std::fmt::Debug for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::DoublePrecisionFloat => write!(f, "sd"),
            Keyword::SinglePrecisionFloat => write!(f, "ss"),
            Keyword::Signed => todo!(),
            Keyword::ZeroExtened => todo!(),
            Keyword::E => write!(f, "e"),
            Keyword::G => write!(f, "g"),
            Keyword::L => write!(f, "l"),
            Keyword::NE => write!(f, "ne"),
            Keyword::GE => write!(f, "ge"),
            Keyword::LE => write!(f, "le"),
        }
    }
}

impl Keyword {
    fn parse<'a>(token: &'a str) -> Option<Self> {
        match token {
            "single-precision-float" => Some(Self::SinglePrecisionFloat),
            "double-precision-float" => Some(Self::DoublePrecisionFloat),
            "signed" => Some(Self::Signed),
            "zero-extended" => Some(Self::ZeroExtened),
            "=" => Some(Self::E),
            "!=" => Some(Self::NE),
            "<" => Some(Self::L),
            "<=" => Some(Self::LE),
            ">" => Some(Self::G),
            ">=" => Some(Self::GE),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub(crate) enum Preposition {
    To,
    From,
    By,
    As,
    With,
    If, // unofficial
}

impl Preposition {
    fn parse<'a>(token: &'a str) -> Option<Self> {
        match token {
            "to" => Some(Self::To),
            "from" => Some(Self::From),
            "by" => Some(Self::By),
            "as" => Some(Self::As),
            "with" => Some(Self::With),
            "if" => Some(Self::If),
            _ => None,
        }
    }
}

pub(crate) type Label<'a> = &'a str;

#[derive(Clone, Copy)]
pub struct Memory {
    pub(crate) base: Option<Register>,
    pub(crate) displacement: Option<i64>,
    pub(crate) index: Option<Register>,
    pub(crate) scale: Option<usize>,
    pub(crate) size: usize,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            base: None,
            displacement: None,
            index: None,
            scale: None,
            size: 0,
        }
    }

    // base = reg
    // index = reg
    // scale = 1|2|4|8
    // disp = num
    // mem = '['((base ('+' index ('*' scale)?)? ('+' disp)? ]) | (index '*' scale '+')? disp]))
    pub fn parse<'a>(mut self, token: &'a str) -> Result<Self> {
        let tokens = token
            .replace("@[", "[ ")
            .replace("]", " ]")
            .replace("*", " * ")
            .replace("+", " + ")
            .replace("-", " - ");
        let token_seq = tokens.split_ascii_whitespace().collect::<Vec<&str>>();
        let mut pos = 1;
        self.parse_base(&token_seq, &mut pos)?;
        // println!("{}", token);
        // println!("{:?}", self);
        Ok(self)
    }

    fn parse_base(&mut self, token_seq: &Vec<&str>, pos: &mut usize) -> Result<()> {
        if Register::is_reg(token_seq[*pos]) {
            if token_seq[*pos + 1] == "]"
                || token_seq[*pos + 1] == "+"
                || token_seq[*pos + 1] == "-"
            {
                self.base = Register::parse(token_seq[*pos]);
                *pos += 1;
            }
        }
        self.parse_idx_scl(token_seq, pos)
    }

    fn parse_idx_scl(&mut self, token_seq: &Vec<&str>, pos: &mut usize) -> Result<()> {
        if token_seq[*pos] == "+" {
            *pos += 1;
        }
        if Register::is_reg(token_seq[*pos]) {
            if token_seq[*pos + 1] == "]"
                || token_seq[*pos + 1] == "+"
                || token_seq[*pos + 1] == "-"
            {
                self.index = Register::parse(token_seq[*pos]);
                *pos += 1;
            } else if token_seq[*pos + 1] == "*" {
                self.index = Register::parse(token_seq[*pos]);
                *pos += 2;
                // todo: emit error data
                self.scale = Some(token_seq[*pos].parse::<usize>().or_else(|_| Err(()))?);
                *pos += 1;
            }
        }
        self.parse_disp(token_seq, pos)
    }

    fn parse_disp(&mut self, token_seq: &Vec<&str>, pos: &mut usize) -> Result<()> {
        let sgn: i64;
        if token_seq[*pos] == "+" {
            *pos += 1;
            sgn = 1;
        } else if token_seq[*pos] == "-" {
            *pos += 1;
            sgn = -1;
        } else if token_seq[*pos] == "]" {
            return Ok(());
        } else {
            sgn = 1
        }
        // todo: emit error data
        self.displacement = Some(sgn * token_seq[*pos].parse::<i64>().or_else(|_| Err(()))?);
        Ok(())
    }
}

impl std::fmt::Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

        if let Some(disp) = self.displacement {
            if disp > 0 {
                if count > 0 {
                    write!(f, " + ")?;
                }
                write!(f, "{}", disp)?;
            } else {
                write!(f, "{}", disp)?;
            }
        }
        write!(f, "]")
    }
}
