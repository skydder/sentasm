use std::fmt::Debug;

use super::{Loc, Result, Tonkenizer};


// use someday
const REG8: &[&'static str] = &["al", "bl", "cl", "dl", "dil", "sil", "bpl", "spl", "r8b", "r9b", "r10b", "r11b", "r12b", "r13b","r14b", "r15b"];
const REG16: &[&'static str] = &["ax", "bx", "cx", "dx", "di", "si", "bp", "sp", "r8w", "r9w", "r10w", "r11w", "r12w", "r13w", "r14w", "r15w"];
const REG32: &[&'static str] = &["eax", "ebx", "ecx", "edx", "edi", "esi", "ebp", "esp", "r8d", "r9d", "r10d", "r11d", "r12d", "r13d", "r14d", "r15d"];
const REG64: &[&'static str] = &["rax", "rbx", "rcx", "rdx", "rdi", "rsi", "rbp", "rsp", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"];
const XMM: &[&'static str] = &["xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7"];

#[derive(Debug)]
pub struct DataSet<'a> {
    pub data:Data<'a>,
    pub loc: Loc<'a>,
}

#[derive(Debug)]
pub enum Data<'a> {
    Verb(Verb),
    Register(Register),
    Prepositon(Preposition),
    Immediate(i64),
    MemoryLP,
    MemoryRP,
    MemorySym(&'a str),
    // Memory(Memory<'a>),
    Label(Label<'a>),
    LabelDef,
    LabelSpecial,
    Keyword(Keyword),
}

impl<'a> DataSet<'a> {
    pub fn new(token: &'a str, loc: Loc<'a>) -> Self {
        Self{ data: Data::parse(token), loc}
    }
    pub fn expect_object(self) -> Result<Self> {
        match self.data {
            Data::Immediate(_) | Data::Keyword(_) | Data::Label(_) | Data::Register(_) => Ok(self),
            _ => {
                eprintln!("expected object, but found other type token");
                Err(())
            }
        }
    }

    pub fn expect_register(self) -> Result<Self> {
        match self.data {
            Data::Register(_) => Ok(self),
            _ => {
                eprintln!("expected register, but found other type token");
                Err(())
            }
        }
    }

    pub fn expect_immediate(self) -> Result<Self> {
        match self.data {
            Data::Immediate(_) => Ok(self),
            _ => {
                eprintln!("expected immediate, but found other type token");
                Err(())
            }
        }
    }
}

impl<'a> Data<'a> {
    fn parse(token: &'a str) -> Data {
        if token == "@[" {
            Data::MemoryLP
        } else if token == "]" {
            Data::MemoryRP
        } else if token == "#" {
            Data::LabelSpecial
        }else if token == ":" {
            Data::LabelDef
        }else if let Some(v) = Verb::parse(token) {
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
    pub fn reg(self) -> Result<Register>{
        match self {
            Self::Register(reg) => Ok(reg),
            _ => {
                eprintln!("expected register, but found other token");
                Err(())
        },
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
}

# [derive(Debug)]
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
    // intransitive verbs
    Return,
    Leave,
    NoOperation,
    SystemCall,
    Halt,
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
            "xor" =>  Some(Self::Xor),
            "not" => Some(Self::Not),
            "negate" => Some(Self::Negate),
            "shift-right" => Some(Self::ShiftRight),
            "shift-left" =>Some(Self::ShiftLeft),
            "call" => Some(Verb::Call),
            "compare" =>  Some(Self::Compare),

            "return" => Some(Self::Return),
            "halt" => Some(Self::Halt),
            "leave" => Some(Self::Leave),
            "no-operation" => Some(Self::NoOperation),
            "systemcall" => Some(Self::SystemCall),
            _ => None,
        }
    }
}

// temporary implementation
#[derive(Debug)]
pub(crate) enum Register {
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

impl Register {
    fn parse<'a>(token: &'a str) -> Option<Self> {
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

#[derive(Debug)]
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

impl Keyword {
    fn parse<'a>(token: &'a str) ->  Option<Self> {
        match token {
            "single-precision-float" => Some(Self::SinglePrecisionFloat),
            "double-precision-float" => Some(Self::DoublePrecisionFloat),
            "signed" =>  Some(Self::Signed),
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

#[derive(PartialEq, Eq, Hash, Debug)]
pub(crate) enum Preposition {
    To,
    From,
    By,
    As,
    With,
    If, // unofficial
}

impl Preposition {
    fn parse<'a>(token: &'a str) -> Option<Self>
    {
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

pub(crate) struct Memory<'a> {
    pub(crate) base:Option<(Register, Loc<'a>)>,
    pub(crate) displacement:Option<(i64, Loc<'a>)>,
    pub(crate) index:Option<(Register, Loc<'a>)>,
    pub(crate) scale: Option<(usize, Loc<'a>)>,
}
