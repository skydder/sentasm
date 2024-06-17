// this code is a rough sketch
mod data;
use data::{AsmError,Token, Sentence};

fn main() -> Result<(), AsmError>{
    let add = "add 1 to eax";
    let token = Token::tokenize(add);
    println!("{}",Sentence::read(token)?.parse()?);

    let substract = "substract 1 from eax";
    let token = Token::tokenize(substract);
    println!("{}",Sentence::read(token)?.parse()?);
    Ok(())
}