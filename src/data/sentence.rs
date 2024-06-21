use super::{AsmError, Parse, Token};

use std::collections::HashMap;
use std::fmt;

pub(crate) enum Verb {
    Add,
    Substract,
    Multiply,
    Divide,
    Move,
}

impl Parse for Verb {
    fn parse(token: &mut Token) -> Result<Self, AsmError>
    where
        Self: Sized,
    {
        match token.inspect() {
            "add" => {
                token.next();
                Ok(Self::Add)
            }
            "substract" => {
                token.next();
                Ok(Self::Substract)
            }
            "multiply" => {
                token.next();
                Ok(Self::Multiply)
            }
            "divide" => {
                token.next();
                Ok(Self::Divide)
            }
            "move" => {
                token.next();
                Ok(Self::Move)
            }

            s => Err(AsmError::SyntaxError(format!(
                ":{}: expected a verb, but found '{}'",
                token.location() + 1,
                s
            ))),
        }
    }
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

impl Parse for Register {
    fn parse(token: &mut Token) -> Result<Register, AsmError> {
        let reg = match token.inspect() {
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
        }?;
        // bit inefficient
        token.next();
        Ok(reg)
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

pub(crate) enum Object {
    Reg(Register),
    Imm(i64),
}

impl Parse for Object {
    fn parse(token: &mut Token) -> Result<Self, AsmError>
    where
        Self: Sized,
    {
        match token.inspect() {
            s => match s.parse::<i64>() {
                Ok(num) => {
                    token.next();
                    Ok(Self::Imm(num))
                }
                Err(_) => Ok(Self::Reg(Register::parse(token)?)),
            },
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Imm(i) => write!(f, "{}", i),
            Self::Reg(reg) => write!(f, "{}", reg),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub(crate) enum Preposition {
    To,
    From,
    By,
}

impl Parse for Preposition {
    fn parse(token: &mut Token) -> Result<Self, AsmError>
    where
        Self: Sized,
    {
        match token.inspect() {
            "to" => {
                token.next();
                Ok(Self::To)
            }
            "from" => {
                token.next();
                Ok(Self::From)
            }
            "by" => {
                token.next();
                Ok(Self::By)
            }
            s => Err(AsmError::SyntaxError(format!(
                ":{}: expected object, but found {}",
                token.location() + 1,
                s
            ))),
        }
    }
}

type PrepositionPhrase = (Preposition, Object);

fn read_preposition_phrase<'a>(token: &mut Token<'a>) -> Result<PrepositionPhrase, AsmError> {
    let prep = Preposition::parse(token)?;
    let object = Object::parse(token)?;
    Ok((prep, object))
}

pub(crate) struct PrepositionPhrases {
    pub(crate) phrases: HashMap<Preposition, Object>,
}

impl Parse for PrepositionPhrases {
    fn parse(token: &mut Token) -> Result<Self, AsmError>
    where
        Self: Sized,
    {
        let mut map: HashMap<Preposition, Object> = HashMap::new();
        while token.len != 0 {
            let (prep, obj) = read_preposition_phrase(token)?;
            map.insert(prep, obj);
        }
        Ok(Self { phrases: map })
    }
}

pub struct Sentence {
    pub(crate) verb: Verb,
    pub(crate) object: Object,
    pub(crate) prepositional_phrases: PrepositionPhrases,
}

impl Parse for Sentence {
    fn parse(token: &mut Token) -> Result<Self, AsmError>
    where
        Self: Sized,
    {
        let verb = Verb::parse(token)?;
        let object = Object::parse(token)?;
        let pps = PrepositionPhrases::parse(token)?;
        assert!(token.seq.len() == token.location() + token.len);
        assert!(token.len == 0);
        Ok(Sentence {
            verb: verb,
            object: object,
            prepositional_phrases: pps,
        })
    }
}
