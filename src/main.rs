// this code is a rough sketch
mod data;
use data::{AsmError, Parse, Sentence, Token};

fn main() -> Result<(), AsmError> {
    let add = "add 1 to eax\n";
    println!("{}", codegen(add)?);

    let sub = "substract 1 from ax\n";
    println!("{}", codegen(sub)?);

    let mul = "multiply eax by ebx\n";
    println!("{}", codegen(mul)?);

    // let div = "divide eax by ebx\n";
    // println!("{}", codegen(div)?);

    Ok(())
}

fn codegen<'a>(s: &'a str) -> Result<String, AsmError> {
    let mut token = Token::tokenize(s);
    let code = format!("\t{}", Sentence::parse(&mut token)?.codegen()?);
    Ok(code)
}