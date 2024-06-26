// this code is a rough sketch
mod data;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt::Write;

use data::{codegen, AsmError, Sentence, Token};

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
    let mut token = Token::tokenize(s);
    let mut sentence = Sentence::parse(&mut token)?;
    let code = format!("\t{}", codegen(&mut sentence)?);
    Ok(code)
}

fn compile_file<R>(reader: R) -> Result<String, AsmError> 
where R: BufRead
{
    let mut code :String = String::new();
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        writeln!(code, "{}", compile(&line)?).unwrap();
    }
    Ok(code)
}