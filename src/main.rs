// this code is a rough sketch


fn main() -> Result<(), AsmError>{
    let add = "add 1 to eax";
    let token = Token::tokenize(add);
    println!("{}",parse_add(read_sentence(token)?)?);
    Ok(())
}

#[derive(Debug)]
enum AsmError {
    SyntaxError(String),
}

struct Token<'a> {
    seq: Vec<&'a str>,
    location: usize,
}

impl<'a> Token<'a> {
    // too ad hoc!!
    fn tokenize(s: &'a str) -> Self {
        // todo: make this function general
        Self {
            seq: s.split_whitespace().collect::<Vec<&str>>(),
            location: 0,
        }
    }

    fn inspect(&self) -> &'a str {
        return self.seq[self.location];
    }

    fn pop_left(&mut self) -> &'a str{
        let ret = self.inspect();
        self.next();
        ret
    }

    fn next(&mut self) {
        self.location += 1;
    }
}

struct Sentence<'a> {
    verb: &'a str,
    object: Object,
    prepositional_phrases: PrepositionPhrase, // at this moment
}

enum Preposition {
    To,
}

type PrepositionPhrase = (Preposition, Object);

#[derive(Debug)]
enum Object {
    Reg,
    Imm,
}

fn read_object<'a>(token: &Token<'a>) -> Result<Object, AsmError> {
    // at this moment
    match token.inspect() {
        "eax" => Ok(Object::Reg),
        s => match s.parse::<i64>() {
            Ok(num) => Ok(Object::Imm),
            Err(_) => Err(AsmError::SyntaxError(format!("expected object, but found {}", s)))
        }
    }
}

fn read_preposition_phrase<'a>(mut token: Token<'a>) -> Result<PrepositionPhrase, AsmError> {
    let prep = match token.inspect() {
        "to" => Ok(Preposition::To),
        s => Err(AsmError::SyntaxError(format!("expected object, but found {}", s)))
    }?;
    token.next();
    let object = read_object(&token)?;
    token.next();
    Ok((prep, object))
}

fn read_sentence<'a>(mut token: Token<'a>) -> Result<Sentence<'a>, AsmError> {
    let verb = token.pop_left();
    let object = read_object(&token)?;
    token.next();
    let pp = read_preposition_phrase(token)?;
    Ok(Sentence { verb: verb, object: object, prepositional_phrases: pp })
}

fn parse_add<'a>(sentence: Sentence<'a>) -> Result<String, AsmError> {
    match sentence {
        Sentence {
            verb: "add",
            object: o,
            prepositional_phrases: (Preposition::To, po)
        } => {
            let pp = format!("to {:?}", po);
            Ok(format!("{v} {o:?} {pp}", v = sentence.verb, o = o, pp = pp))
        },
        _ => Err(AsmError::SyntaxError("expected add instruction, but found other".to_string()))?
    }
    
}

// add <object> to <object>


// struct Add {
//     object: Object,
//     to: Object,
// }

// fn read_to<'a>(mut token:Token<'a>) -> Result<Object, AsmError> {
//     let to = token.pop_left();
//     if to == "to" {
//         Ok(token.pop_left())
//     } else {
//         Err(AsmError::SyntaxError(format!("expected 'to', but found '{}'", to)))
//     }
// }

// fn read_add<'a>(mut token:Token<'a>) -> Result<Add<'a>, AsmError> {
//     let object: Object;
//     let to: Object;
//     let add = token.pop_left();
//     if add == "add" {
//         object = token.pop_left();
//         to = read_to(token)?;
//         Ok(Add {
//             object: object,
//             to: to
//         })
//     } else {
//         Err(AsmError::SyntaxError(format!("expected 'add', but found '{}'", add)))
//     }
// }

// fn gen_add<'a>(add: Add<'a>) -> String {
//     format!("add {}, {}", add.to, add.object)
// }
