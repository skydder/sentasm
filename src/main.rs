fn main() {
    let add = "add 1 to eax";
    let mut token = tokenize(add);
    println!("{:?}", read_add(token));
}

struct Token<'a> {
    seq: Vec<&'a str>,
    location: usize,
}

impl<'a> Token<'a> {
    fn get(&self) -> &'a str{
        return self.seq[self.location];
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

fn read_to<'a>(mut token:Token<'a>) -> Object<'a> {
    if token.get() == "to" {
        token.next();
        return token.get();
    }
    todo!();
}

fn read_add<'a>(mut token:Token<'a>) -> Add<'a> {
    let mut object: Object<'a>;
    let mut to: Object<'a>;
    if token.get() == "add" {
        token.next();
        object = token.get();
        token.next();
        to = read_to(token);
        return Add{
            object: object,
            to: to
        };
    }
    todo!();
}
