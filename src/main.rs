// this code is a rough sketch
mod data;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt::Write;

use data::sentence::Paragraph;
use data::{codegen, AsmError, Parse, Sentence, Token};

fn main() -> Result<(), AsmError> {
    println!("{}", compile_file(BufReader::new(File::open(read_args()?).unwrap()))?);
    Ok(())
}

fn read_args() -> Result<String, AsmError> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() != 1 {
        return Err(AsmError::SyntaxError(format!("args error")));
    }
    let path = &args[0];
    Ok(path.to_string())
}

fn compile<'a>(s: &'a str) -> Result<String, AsmError> {
    let code = codegen(Paragraph::parse(s)?)?;
    Ok(code)
}

fn compile_file<R>(mut reader: R) -> Result<String, AsmError> 
where R: BufRead
{
    let mut code :String = String::new();
    // let mut sourse :String = String::new();
    // reader.read_to_string(&mut sourse).unwrap();

    // code.push_str(&compile(&code)?);
    Ok(code)
}