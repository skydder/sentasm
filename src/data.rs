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
pub enum Object {
    Reg,
    Imm(i64),
}

impl Object {
    fn read<'a>(token: &mut Token<'a>) -> Result<Self, AsmError> {
        let obj = match token.inspect() {
            "eax" => Ok(Self::Reg),
            s => match s.parse::<i64>() {
                Ok(num) => Ok(Self::Imm(num)),
                Err(_) => Err(AsmError::SyntaxError(format!(":{}: expected object, but found {}", token.location + 1, s)))
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
    pub fn read<'a>(mut token: Token<'a>) -> Result<Self, AsmError> {
        let verb = Verb::read(&mut token)?;
        let object = Object::read(&mut token)?;
        let pp = read_preposition_phrase(&mut token)?;
        assert!(token.seq.len() == token.location + token.len);
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
