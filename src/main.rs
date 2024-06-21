// this code is a rough sketch
mod data;
use data::{codegen, AsmError, Parse, Sentence, Token};

fn main() -> Result<(), AsmError> {
    let add = "add 1 to eax\n";
    println!("{}", compile(add)?);

    let sub = "substract 1 from ax\n";
    println!("{}", compile(sub)?);

    let mul = "multiply eax by ebx\n";
    println!("{}", compile(mul)?);

    let div = "divide eax \n";
    println!("{}", compile(div)?);

    let mov = "move 1 to eax\n";
    println!("{}", compile(mov)?);

    Ok(())
}

fn compile<'a>(s: &'a str) -> Result<String, AsmError> {
    let mut token = Token::tokenize(s);
    let mut sentence = Sentence::parse(&mut token)?;
    let code = format!("\t{}", codegen(&mut sentence)?);
    Ok(code)
}
