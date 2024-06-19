#[derive(Debug)]
pub enum AsmError {
    SyntaxError(String),
}

pub struct Token<'a> {
    seq: &'a str,
    location: usize,
    len: usize,
}

impl<'a> Token<'a> {
    pub fn tokenize(s: &'a str) -> Self {
        let mut new = Self {
            seq: s,
            location: 0,
            len: 0
        };

        new.len = new.calculate_len();
        new
    }

    pub fn inspect(&mut self) -> &'a str {
        return &self.seq[self.location..self.location + self.len];
    }

    fn calculate_len(&self) -> usize {
        let mut len = 0;
        while !self.seq.chars().nth(self.location + len).unwrap_or(' ').is_whitespace() {
            len += 1;
        }
        len
    }

    pub fn next(&mut self) {
        self.location += self.len;
        while self.seq.chars().nth(self.location).unwrap_or('a').is_whitespace() {
            self.location += 1;
        }
        self.len = self.calculate_len();
    }
}

#[derive(Debug)]
enum Verb {
    Add,
    Substract,
}

impl Verb {
    fn read<'a>(token: &mut Token<'a>) -> Result<Self, AsmError> {
        let verb = match token.inspect() {
            "add" => Ok(Self::Add),
            "substract" => Ok(Self::Substract),
            s => Err(AsmError::SyntaxError(format!(":{}: expected a verb, but found '{}'", token.location + 1, s)))
        }?;
        token.next();
        Ok(verb)
    }
}

#[derive(Debug)]
enum Register {
    // general purpose regiser
    AL, BL, CL, DL, DIL, SIL, BPL, SPL, R8B, R9B, R10B, R11B, R12B, R13B, R14B, R15B, // byte
    AX, BX, CX, DX, DI, SI, BP, SP, R8W, R9W, R10W, R11W, R12W, R13W, R14W, R15W, // word
    EAX, EBX, ECX, EDX, EDI, ESI, EBP, ESP, R8D, R9D, R10D, R11D, R12D, R13D, R14D, R15D, // doubleword
    RAX, RBX, RCX, RDX, RDI, RSI, RBP, RSP, R8, R9, R10, R11, R12, R13, R14, R15, //  quadword
}

impl Register {
    fn read<'a>(s:&'a str) -> Result<Self, AsmError> {
        match s {
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
            _ => Err(AsmError::SyntaxError(format!("expected object, but found {}", s))),
        }
    }
}

#[derive(Debug)]
pub enum Object {
    Reg(Register),
    Imm(i64),
}

impl Object {
    fn read<'a>(token: &mut Token<'a>) -> Result<Self, AsmError> {
        let obj = match token.inspect() {
            s => match s.parse::<i64>() {
                Ok(num) => Ok(Self::Imm(num)),
                Err(_) => Ok(Self::Reg(Register::read(s)?))
            }
        }?;
    
        token.next();
        Ok(obj)
    }
}

enum Preposition {
    To,
    From,
}

impl Preposition {
    fn read<'a>(token: &mut Token<'a>) -> Result<Self, AsmError> {
        let prep = match token.inspect() {
            "to" => Ok(Self::To),
            "from" => Ok(Self::From),
            s => Err(AsmError::SyntaxError(format!(":{}: expected object, but found {}", token.location + 1, s)))
        }?;
        token.next();
        Ok(prep)
    }
}

type PrepositionPhrase = (Preposition, Object);


fn read_preposition_phrase<'a>(token: &mut Token<'a>) -> Result<PrepositionPhrase, AsmError> {
    let prep = Preposition::read(token)?;
    let object = Object::read(token)?;
    Ok((prep, object))
}

pub struct Sentence {
    verb: Verb,
    object: Object,
    prepositional_phrases: PrepositionPhrase,
}

impl Sentence {
    pub fn read<'a>(mut token:Token<'a>) -> Result<Self, AsmError> {
        let verb = Verb::read(&mut token)?;
        let object = Object::read(&mut token)?;
        let pp = read_preposition_phrase(&mut token)?;
        assert!(token.seq.len() == token.location + token.len);
        assert!(token.len == 0);
        Ok(Sentence { verb: verb, object: object, prepositional_phrases: pp })
    }

    pub fn parse<'a>(self) -> Result<String, AsmError> {
        match self {
            Sentence {
                verb: Verb::Add,
                object: o,
                prepositional_phrases: (Preposition::To, po)
            } => {
                let pp = format!("to {:?}", po);
                Ok(format!("{v} {o:?} {pp}", v = "add", o = o, pp = pp))
            },

            Sentence {
                verb: Verb::Substract,
                object: o,
                prepositional_phrases: (Preposition::From, po)
            } => {
                let pp = format!("from {:?}", po);
                Ok(format!("{v} {o:?} {pp}", v = "substract", o = o, pp = pp))
            },
    
            _ => Err(AsmError::SyntaxError("expected  instruction, but found other".to_string()))?
        }
        
    }

}
