// this code is a rough sketch

//use std::fmt::Error;

fn main() -> Result<(), AsmError>{
    let add = "add 1 to eax";
    let token = tokenize(add);
    println!("{}", gen_add(read_add(token)?));
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
    fn get(&self) -> &'a str {
        return self.seq[self.location];
    }

    fn pop_left(&mut self) -> &'a str{
        let ret = self.get();
        self.next();
        ret
    }

    fn next(&mut self) {
        self.location += 1;
    }
}

// too ad hoc!!
fn tokenize<'a>(s: &'a str) -> Token<'a> {
    // todo: make this function general
    Token {
        seq: s.split_whitespace().collect::<Vec<&str>>(),
        location: 0,
    }
}

type Object<'a> = &'a str;

// add <object> to <object>
#[derive(Debug)]
struct Add<'a> {
    object: Object<'a>,
    to: Object<'a>,
}

fn read_to<'a>(mut token:Token<'a>) -> Result<Object<'a>, AsmError> {
    let to = token.pop_left();
    if to == "to" {
        Ok(token.pop_left())
    } else {
        Err(AsmError::SyntaxError(format!("expected 'to', but found '{}'", to)))
    }
}

fn read_add<'a>(mut token:Token<'a>) -> Result<Add<'a>, AsmError> {
    let object: Object<'a>;
    let to: Object<'a>;
    let add = token.pop_left();
    if add == "add" {
        object = token.pop_left();
        to = read_to(token)?;
        Ok(Add {
            object: object,
            to: to
        })
    } else {
        Err(AsmError::SyntaxError(format!("expected 'add', but found '{}'", add)))
    }
}

fn gen_add<'a>(add: Add<'a>) -> String {
    format!("add {}, {}", add.to, add.object)
}
