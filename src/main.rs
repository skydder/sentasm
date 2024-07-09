// this code is a rough sketch
mod data;

use std::fs::File;
use std::io::{BufRead, BufReader};

use data::{codegen, AsmError, Sentence, Token, TokenLocation};

fn main() {
    match read_args() {
        Ok(file) => compile(&file).unwrap_or_else(|error| println!("{}", error)),
        Err(error) => println!("{}", error)
    }
    
}

fn read_args<'a>() -> Result<String, AsmError<'a>> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() != 1 {
        return Err(AsmError::IOError(format!("args error")));
    }
    let path = &args[0];
    Ok(path.to_string())
}


fn compile(file:&str) -> Result<(), AsmError>{
    let mut code: String = String::new();
    for (ln, line_result) in BufReader::new(File::open(file).unwrap()).lines().enumerate() {
        let line = line_result.unwrap();
        let mut token = Token::tokenize(&line, TokenLocation::new(file, ln, 0));
        let mut sentence = Sentence::parse(&mut token)?;
        code.push_str(&codegen(&mut sentence)?);
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("{}", code);
    Ok(())
}