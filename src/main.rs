<<<<<<< HEAD
// this code is a rough sketch
mod data;

use std::fs::File;
use std::io::{BufRead, BufReader};

use data::{AsmError, Token, TokenLocation, compile};
=======
use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

mod asmblr;

use asmblr::{codegen, Code, Loc, Tonkenizer};
>>>>>>> new

fn main() {
    match read_args() {
        Ok(file) => compile_file(&file),
<<<<<<< HEAD
        Err(error) => println!("{}", error)
    }
    
}

fn read_args<'a>() -> Result<String, AsmError<'a>> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() != 1 {
        return Err(AsmError::IOError(format!("args error")));
=======
        Err(_) => (),
    }
}

fn read_args<'a>() -> Result<String, ()> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() != 1 {
        panic!();
>>>>>>> new
    }
    let path = &args[0];
    Ok(path.to_string())
}

<<<<<<< HEAD

fn compile_file(file:&str) {
    let mut code: String = String::new();
    for (ln, line_result) in BufReader::new(File::open(file).unwrap()).lines().enumerate() {
        let line = line_result.unwrap().clone();
        let token = Box::new(Token::tokenize(&line, TokenLocation::new(file, ln, 0)));
        match compile(&token, &mut code) {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("{}", code);
}
=======
fn compile_file(file: &str) {
    for (ln, line_result) in BufReader::new(File::open(file).unwrap())
        .lines()
        .enumerate()
    {
        let line = line_result.unwrap().clone();
        let _tokenizer = Tonkenizer::new(&line, Loc::new(file, ln, 0));
        // println!("{:?}", Code::parse(&_tokenizer));
        println!(
            "{}",
            codegen(Code::parse(&_tokenizer).unwrap_or_else(|_| exit(404)))
                .unwrap_or_else(|_| exit(404))
        );
    }
}
>>>>>>> new
