use super::{AsmError, Token, TokenLocation};

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

type Label<'a> = &'a str;
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

#[derive(Debug)]
pub(crate) struct Memory {
    base: Register, // Option<Register>,
                    // Index: Option<Register>,
                    // Scale: Option<u8>
}

#[derive(Debug)]
pub(crate) enum Object<'a> {
    Reg(Register),
    Imm(i64),
    Mem(Memory),
    Label(Label<'a>),
    Keyword(Keyword)
}
#[derive(Debug)]
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

    XMM0,
    XMM1,
    XMM2,
    XMM3,
    XMM4,
    XMM5,
    XMM6,
    XMM7,
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

pub enum Sentence<'a> {
    Sentence {
        verb: Box<(Verb, TokenLocation<'a>)>,
        object: Option<Box<(Object<'a>, TokenLocation<'a>)>>,
        prepositional_phrases: PrepositionPhrases<'a>,
    },
    LabelDefinition(Box<(Label<'a>, TokenLocation<'a>)>),
    Null,
}

# [derive(Debug)]
pub(crate) enum _TokenKind<'a> {
    Verb(Verb),
    Object(Object<'a>),
    Preposition(Preposition),
    LabelDef(Label<'a>),
    EOL,
}

pub(crate) struct TokenKind<'a> {
    token: _TokenKind<'a>,
    location: TokenLocation<'a>
}

impl<'a> Token<'a> {
    pub(crate) fn inspect(&self) -> Result<TokenKind<'a>, AsmError> {
        let tok = self._inspect();
        // println!("{:?} ; {} ", self, tok);
        if let Some(v) = Verb::parse(tok) {
            Ok(TokenKind::new(_TokenKind::Verb(v), *self.location.borrow()))
        } else if let Some(o) = Object::parse(tok) {
            Ok(TokenKind::new(_TokenKind::Object(o),*self.location.borrow()))
        } else if let Some(pp) = Preposition::parse(tok) {
            Ok(TokenKind::new(_TokenKind::Preposition(pp), *self.location.borrow()))
        } else if tok.ends_with(':') {
            Ok(TokenKind::new(_TokenKind::LabelDef(tok.strip_suffix(':').unwrap()), *self.location.borrow()))
        } else if tok.starts_with('<') & tok.starts_with('<') {
            Ok(TokenKind::new(_TokenKind::LabelDef(&tok[1..tok.len() - 1]), *self.location.borrow()))
        } else if self.is_end() {
            Ok(TokenKind::new(_TokenKind::EOL, *self.location.borrow()))
        } else {
            Err(AsmError::SyntaxError(*self.location.borrow(), format!("unexpected token")))
        }
    }
}

impl<'a> TokenKind<'a> {
    fn new(token:_TokenKind<'a>, loc: TokenLocation<'a>) -> Self {
        Self { token, location: loc}
    }

    fn expect_verb(self) -> Result<(Verb, TokenLocation<'a>), AsmError<'a>> {
        if let _TokenKind::Verb(v) = self.token {
            Ok((v, self.location))
        } else {
            Err(AsmError::SyntaxError(self.location,
                format!(
                "expected a verb, but found other: {:?}", self.token
            )))
        }
    }
    fn expect_object(self) -> Result<(Object<'a>, TokenLocation<'a>), AsmError<'a>> {
        if let _TokenKind::Object(o) = self.token {
            Ok((o, self.location))
        } else {
            Err(AsmError::SyntaxError(self.location,
                format!(
                "expected a verb, but found other: {:?}", self.token
            )))
        }
    }

    fn expect_preposition(self) -> Result<(Preposition, TokenLocation<'a>), AsmError<'a>> {
        if let _TokenKind::Preposition(pp) = self.token {
            Ok((pp, self.location))
        } else {
            Err(AsmError::SyntaxError(self.location,
                format!(
                "expected a verb, but found other: {:?}", self.token
            )))
        }
    }

    fn expect_label_def(self) -> Result<(Label<'a>, TokenLocation<'a>), AsmError<'a>> {
        if let _TokenKind::LabelDef(l) = self.token {
            Ok((l, self.location))
        } else {
            Err(AsmError::SyntaxError(self.location,
                format!(
                "expected a verb, but found other: {:?}", self.token
            )))
        }
    }
}

impl Verb {
    fn parse<'a>(token: &'a str) -> Option<Self>
    where
        Self: Sized,
    {
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
            Register::XMM0 => "xmm0",
            Register::XMM1 => "xmm1",
            Register::XMM2 => "xmm2",
            Register::XMM3 => "xmm3",
            Register::XMM4 => "xmm4",
            Register::XMM5 => "xmm5",
            Register::XMM6 => "xmm6",
            Register::XMM7 => "xmm7",
        };
        write!(f, "{}", reg)
    }
}

impl<'b> Object<'b> {
    fn parse(token: &'b str) -> Option<Self>
    where
        Self: Sized,
    {
        // little weird?
        if token.starts_with('[') {
            // base + idx * scale + dis
            if let Some(mem) = Memory::parse(process(token)) {
                return Some(Self::Mem(mem));
            } else {
                return None;
            }
        } else if let Ok(num) = token.parse::<i64>() {
            return Some(Self::Imm(num));
        } else if let Some(reg) = Register::parse(token) {
            return Some(Self::Reg(reg));
        } else if let Some(key) = Keyword::parse(token) {
            return Some(Self::Keyword(key));
        } else if !(token.ends_with(':') | token.is_empty() | Preposition::is_prep(token) | token.starts_with('<') | token.starts_with('<')) {
            return Some(Self::Label(&token[..token.len()]));
        } else {
            None
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
    fn parse(token: Vec<String>) -> Option<Self> {
        if let Some(reg) = Register::parse(&token[1]) {
            return Some(Memory { base: reg });
        } else {
            None
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
            Self::Label(label) => write!(f, "{}", label),
            Self::Keyword(_) => write!(f, ""),
        }
    }
}

impl Preposition {
    fn parse<'a>(token: &'a str) -> Option<Self>
    where
        Self: Sized,
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
    fn is_prep(token: &str) -> bool {
        Self::parse(token).is_some()
    }
}

pub(crate) struct PrepositionPhrases<'a> {
    phrases: RefCell<HashMap<Preposition, (Object<'a>, TokenLocation<'a>)>>,
}

impl<'a> PrepositionPhrases<'a> {
    fn parse(token: &'a Token) -> Result<Self, AsmError<'a>>
    where
        Self: Sized,
    {
        let mut map: HashMap<Preposition, (Object<'a>, TokenLocation<'a>)> = HashMap::new();

        while !token.is_end() {
            let prep = token.inspect()?.expect_preposition()?;
            token.next();
            let obj = token.inspect()?.expect_object()?;
            token.next();
            map.insert(prep.0, obj);
        }
        Ok(Self { phrases: RefCell::new(map) })
    }

    pub(crate) fn consume(&self, pp: Preposition) -> Option<(Object<'a>, TokenLocation<'a>)> {
        self.phrases.borrow_mut().remove(&pp)
    }

    pub(crate) fn have(&self, pp: Preposition) -> bool {
        self.phrases.borrow().contains_key(&pp)
    }

    pub(crate) fn have_no_phrases(&self) -> bool {
        self.phrases.borrow().is_empty()
    }
}

impl<'a> Sentence<'a> {
    pub fn parse(token: &'a Token<'a>) -> Result<Self, AsmError<'a>>
    where
        Self: Sized,
    {
        match token.inspect() {
            Ok(TokenKind { token: _TokenKind::Verb(verb), location }) => {
                token.next();
                // let object = if let Ok(obj) = token.inspect()?.expect_object() {
                //     token.next();
                //     Some(Box::new(obj))
                // } else {
                //     None
                // };
                let object =  match token.inspect()?.expect_object().ok() {
                    Some(obj) => {
                        token.next();
                        Some(Box::new(obj))
                    },
                    None => None
                };

                
                
                            // assert!(token.seq.len() == token.location() + token.len);
                            // assert!(token.len == 0);
                Ok(Self::Sentence {
                    verb: Box::new((verb, location)),
                    object: object,
                    prepositional_phrases: PrepositionPhrases::parse(&token)?,
                })
            }
            Ok(TokenKind{ token: _TokenKind::LabelDef(label), location}) => {
                Ok(Self::LabelDefinition(Box::new((label, location))))
            }
            Ok(TokenKind { token:_TokenKind::EOL, .. }) => Ok(Self::Null),
            _ => Err(AsmError::SyntaxError(*token.location.borrow(),format!("something is wrong1")))
        }
    }
}
