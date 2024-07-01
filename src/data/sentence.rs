use super::{AsmError, Parse, Token};

use std::collections::HashMap;
use std::fmt;

type Label<'a> = &'a str;
pub(crate) enum Verb {
    Add,
    Substract,
    Multiply,
    Divide,
    Move,
    Jump,

    // intransitive verbs
    Return,
    Leave,
    NoOperation,
    SystemCall,
    Halt,
}

pub(crate) struct Memory {
    base: Register, // Option<Register>,
                    // Index: Option<Register>,
                    // Scale: Option<u8>
}
pub(crate) enum Object<'a> {
    Reg(Register),
    Imm(i64),
    Mem(Memory),
    Label(Label<'a>)
}

pub enum Register {
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
}

#[derive(PartialEq, Eq, Hash)]
pub(crate) enum Preposition {
    To,
    From,
    By,
}

pub enum Sentence<'a, 'b> {
    Sentence {
        verb: Verb,
        object: Option<Object<'a>>,
        prepositional_phrases: PrepositionPhrases<'b>,
    },
    LabelDefinition(Label<'a>),
}

pub(crate) enum TokenKind<'a> {
    Verb(Verb),
    Object(Object<'a>),
    Preposition(Preposition),
    Label(Label<'a>),
}

impl<'a> Token<'a> {
    pub(crate) fn inspect(&mut self) -> TokenKind<'a> {
        let tok = self._inspect();
        if let Ok(v) = Verb::parse(tok) {
            TokenKind::Verb(v)
        } else if let Ok(o) = Object::parse(tok) {
            TokenKind::Object(o)
        } else if let Ok(pp) = Preposition::parse(tok) {
            TokenKind::Preposition(pp)
        } else {
            TokenKind::Label(tok)
        }
    }
}

impl<'a> TokenKind<'a> {
    fn expect_verb(self) -> Result<Verb, AsmError> {
        if let Self::Verb(v) = self {
            Ok(v)
        } else {
            Err(AsmError::SyntaxError(format!(
                "expected a verb, but found other"
            )))
        }
    }
    fn expect_object(self) -> Result<Object<'a>, AsmError> {
        if let Self::Object(o) = self {
            Ok(o)
        } else {
            Err(AsmError::SyntaxError(format!(
                "expected an object, but found other"
            )))
        }
    }

    fn expect_preposition(self) -> Result<Preposition, AsmError> {
        if let Self::Preposition(pp) = self {
            Ok(pp)
        } else {
            Err(AsmError::SyntaxError(format!(
                "expected a preposition, but found other"
            )))
        }
    }

    fn expect_label(self) -> Result<Label<'a>, AsmError> {
        if let Self::Label(l) = self {
            Ok(l)
        } else {
            Err(AsmError::SyntaxError(format!(
                "expected a verb, but found other"
            )))
        }
    }
    fn is_object(self) -> bool {
        if let Self::Object(_) = self {
            true
        } else {
            false
        }
    }
}

impl Parse for Verb {
    fn parse<'a>(token: &'a str) -> Result<Self, AsmError>
    where
        Self: Sized,
    {
        match token {
            "add" => Ok(Self::Add),
            "substract" => Ok(Self::Substract),
            "multiply" => Ok(Self::Multiply),
            "divide" => Ok(Self::Divide),
            "move" => Ok(Self::Move),
            "jump" => Ok(Self::Jump),

            "return" => Ok(Self::Return),
            "halt" => Ok(Self::Halt),
            "leave" => Ok(Self::Leave),
            "no-operation" => Ok(Self::NoOperation),
            "systemcall" => Ok(Self::SystemCall),

            s => Err(AsmError::SyntaxError(format!(
                "expected a verb, but found '{}'",
                s
            ))),
        }
    }
}

impl Parse for Register {
    fn parse<'a>(token: &'a str) -> Result<Register, AsmError> {
        match token {
            "al" => Ok(Self::AL),
            "bl" => Ok(Self::BL),
            "cl" => Ok(Self::CL),
            "dl" => Ok(Self::DL),
            "dil" => Ok(Self::DIL),
            "sil" => Ok(Self::SIL),
            "bpl" => Ok(Self::BPL),
            "spl" => Ok(Self::SPL),
            "r8b" => Ok(Self::R8B),
            "r9b" => Ok(Self::R9B),
            "r10b" => Ok(Self::R10B),
            "r11b" => Ok(Self::R11B),
            "r12b" => Ok(Self::R12B),
            "r13b" => Ok(Self::R13B),
            "r14b" => Ok(Self::R14B),
            "r15b" => Ok(Self::R15B),
            "ax" => Ok(Self::AX),
            "bx" => Ok(Self::BX),
            "cx" => Ok(Self::CX),
            "dx" => Ok(Self::DX),
            "di" => Ok(Self::DI),
            "si" => Ok(Self::SI),
            "bp" => Ok(Self::BP),
            "sp" => Ok(Self::SP),
            "r8w" => Ok(Self::R8W),
            "r9w" => Ok(Self::R9W),
            "r10w" => Ok(Self::R10W),
            "r11w" => Ok(Self::R11W),
            "r12w" => Ok(Self::R12W),
            "r13w" => Ok(Self::R13W),
            "r14w" => Ok(Self::R14W),
            "r15w" => Ok(Self::R15W),
            "eax" => Ok(Self::EAX),
            "ebx" => Ok(Self::EBX),
            "ecx" => Ok(Self::ECX),
            "edx" => Ok(Self::EDX),
            "edi" => Ok(Self::EDI),
            "esi" => Ok(Self::ESI),
            "ebp" => Ok(Self::EBP),
            "esp" => Ok(Self::ESP),
            "r8d" => Ok(Self::R8D),
            "r9d" => Ok(Self::R9D),
            "r10d" => Ok(Self::R10D),
            "r11d" => Ok(Self::R11D),
            "r12d" => Ok(Self::R12D),
            "r13d" => Ok(Self::R13D),
            "r14d" => Ok(Self::R14D),
            "r15d" => Ok(Self::R15D),
            "rax" => Ok(Self::RAX),
            "rbx" => Ok(Self::RBX),
            "rcx" => Ok(Self::RCX),
            "rdx" => Ok(Self::RDX),
            "rdi" => Ok(Self::RDI),
            "rsi" => Ok(Self::RSI),
            "rbp" => Ok(Self::RBP),
            "rsp" => Ok(Self::RSP),
            "r8" => Ok(Self::R8),
            "r9" => Ok(Self::R9),
            "r10" => Ok(Self::R10),
            "r11" => Ok(Self::R11),
            "r12" => Ok(Self::R12),
            "r13" => Ok(Self::R13),
            "r14" => Ok(Self::R14),
            "r15" => Ok(Self::R15),
            s => Err(AsmError::SyntaxError(format!(
                "expected object, but found {}",
                s
            ))),
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reg = match self {
            Register::AL => "al",
            Register::BL => "bl",
            Register::CL => "cl",
            Register::DL => "dl",
            Register::DIL => "dil",
            Register::SIL => "sil",
            Register::BPL => "bpl",
            Register::SPL => "spl",
            Register::R8B => "r8b",
            Register::R9B => "r9b",
            Register::R10B => "r10b",
            Register::R11B => "r11b",
            Register::R12B => "r12b",
            Register::R13B => "r13b",
            Register::R14B => "r14b",
            Register::R15B => "r15b",
            Register::AX => "ax",
            Register::BX => "bx",
            Register::CX => "cx",
            Register::DX => "dx",
            Register::DI => "di",
            Register::SI => "si",
            Register::BP => "bp",
            Register::SP => "sp",
            Register::R8W => "r8w",
            Register::R9W => "r9w",
            Register::R10W => "r10w",
            Register::R11W => "r11w",
            Register::R12W => "r12w",
            Register::R13W => "r13w",
            Register::R14W => "r14w",
            Register::R15W => "r15w",
            Register::EAX => "eax",
            Register::EBX => "ebx",
            Register::ECX => "ecx",
            Register::EDX => "edx",
            Register::EDI => "edi",
            Register::ESI => "esi",
            Register::EBP => "ebp",
            Register::ESP => "esp",
            Register::R8D => "r8d",
            Register::R9D => "r9d",
            Register::R10D => "r10d",
            Register::R11D => "r11d",
            Register::R12D => "r12d",
            Register::R13D => "r13d",
            Register::R14D => "r14d",
            Register::R15D => "r15d",
            Register::RAX => "rax",
            Register::RBX => "rbx",
            Register::RCX => "rcx",
            Register::RDX => "rdx",
            Register::RDI => "rdi",
            Register::RSI => "rsi",
            Register::RBP => "rbp",
            Register::RSP => "rsp",
            Register::R8 => "r8",
            Register::R9 => "r9",
            Register::R10 => "r10",
            Register::R11 => "r11",
            Register::R12 => "r12",
            Register::R13 => "r13",
            Register::R14 => "r14",
            Register::R15 => "r15",
        };
        write!(f, "{}", reg)
    }
}

impl<'b> Parse for Object<'b> {
    fn parse<'a>(token: &'a str) -> Result<Self, AsmError>
    where
        Self: Sized,
    {
        // little weird?
        if token.starts_with('[') {
            // base + idx * scale + dis

            return Ok(Self::Mem(Memory::parse(process(token))?));
        } else if let Ok(num) = token.parse::<i64>() {
            return Ok(Self::Imm(num));
        } else if let Ok(reg) = Register::parse(token) {
            return Ok(Self::Reg(reg));
        } else {
            return Ok(Self::Label(&token))
        }
        
    }
}

fn process<'a>(token: &'a str) -> Vec<String> {
    token
        .replace("[", " [ ")
        .replace("]", " ] ")
        .replace("*", " * ")
        .replace("+", " + ")
        .replace("-", " - ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.to_string())
        .collect()
}

impl Memory {
    fn parse(token: Vec<String>) -> Result<Self, AsmError> {
        if let Ok(reg) = Register::parse(&token[1]) {
            return Ok(Memory { base: reg });
        } else {
            Err(AsmError::SyntaxError(format!(
                "expected register, but found other",
            )))
        }
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.base)
    }
}

impl<'a> fmt::Display for Object<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Imm(i) => write!(f, "{}", i),
            Self::Reg(reg) => write!(f, "{}", reg),
            Self::Mem(mem) => write!(f, "{}", mem),
            
            Object::Label(_) => todo!(),
        }
    }
}

impl Parse for Preposition {
    fn parse<'a>(token: &'a str) -> Result<Self, AsmError>
    where
        Self: Sized,
    {
        match token {
            "to" => Ok(Self::To),
            "from" => Ok(Self::From),
            "by" => Ok(Self::By),
            s => Err(AsmError::SyntaxError(format!(
                "expected object, but found {}",
                s
            ))),
        }
    }
}

pub(crate) struct PrepositionPhrases<'a> {
    pub(crate) phrases: HashMap<Preposition, Object<'a>>,
}

impl<'a> PrepositionPhrases<'a> {
    fn parse(token: &'a mut Token) -> Result<Self, AsmError>
    where
        Self: Sized,
    {
        let mut map: HashMap<Preposition, Object> = HashMap::new();
        
        while token.len != 0 {
            let prep = token.inspect().expect_preposition()?;
            token.next();
            let obj = token.inspect().expect_object()?;
            token.next();
            map.insert(prep, obj);
        }
        Ok(Self { phrases: map })
    }
}

impl<'a, 'b> Sentence<'a, 'b> {
    pub fn parse(token: &'b mut Token<'a>) -> Result<Self, AsmError>
    where
        Self: Sized,
    {
        if let Ok(verb) = token.inspect().expect_verb() {
            token.next();
            let object = if token.inspect().is_object() {
                Some(token.inspect().expect_object()?)
            } else {
                None
            };
            token.next();
            let pps = PrepositionPhrases::parse(token)?;
            // assert!(token.seq.len() == token.location() + token.len);
            // assert!(token.len == 0);
            Ok(Self::Sentence {
                verb: verb,
                object,
                prepositional_phrases: pps,
            })
        } else {
            Ok(Self::LabelDefinition(token.inspect().expect_label()?))
        }
    }
}
