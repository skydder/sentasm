#[derive(Debug)]
pub enum AsmError {
    SyntaxError(String),
}

pub struct Token<'a> {
    seq: Vec<&'a str>,
    location: usize,
}

impl<'a> Token<'a> {
    // too ad hoc!!
    pub fn tokenize(s: &'a str) -> Self {
        // todo: make this function general
        Self {
            seq: s.split_whitespace().collect::<Vec<&str>>(),
            location: 0,
        }
    }

    pub fn inspect(&self) -> &'a str {
        return self.seq[self.location];
    }

    pub fn next(&mut self) {
        self.location += 1;
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
            s => Err(AsmError::SyntaxError(format!("expected a verb, but found '{}'", s)))
        }?;
        token.next();
        Ok(verb)
    }
}

#[derive(Debug)]
pub enum Object {
    Reg,
    Imm,
}

impl Object {
    fn read<'a>(token: &mut Token<'a>) -> Result<Self, AsmError> {
        let obj = match token.inspect() {
            "eax" => Ok(Self::Reg),
            s => match s.parse::<i64>() {
                Ok(_) => Ok(Self::Imm),
                Err(_) => Err(AsmError::SyntaxError(format!("expected object, but found {}", s)))
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
            s => Err(AsmError::SyntaxError(format!("expected object, but found {}", s)))
        }?;
        token.next();
        Ok(prep)
    }
}

type PrepositionPhrase = (Preposition, Object);


fn read_preposition_phrase<'a>(mut token: Token<'a>) -> Result<PrepositionPhrase, AsmError> {
    let prep = Preposition::read(&mut token)?;
    let object = Object::read(&mut token)?;
    Ok((prep, object))
}

pub struct Sentence {
    verb: Verb,
    object: Object,
    prepositional_phrases: PrepositionPhrase,
}

impl Sentence {
    pub fn read<'a>(mut token: Token<'a>) -> Result<Self, AsmError> {
        let verb = Verb::read(&mut token)?;
        let object = Object::read(&mut token)?;
        let pp = read_preposition_phrase(token)?;
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
    
            _ => Err(AsmError::SyntaxError("expected add instruction, but found other".to_string()))?
        }
        
    }

}
