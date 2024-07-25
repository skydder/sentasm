use std::{fs::File, io::{BufRead, BufReader}};

mod asmblr;

use asmblr::{Tonkenizer, Loc};

fn main() {
    match read_args() {
        Ok(file) => compile_file(&file),
        Err(_) => ()
    }
}
fn read_args<'a>() -> Result<String, ()> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() != 1 {
        panic!();
    }
    let path = &args[0];
    Ok(path.to_string())
}


fn compile_file(file:&str) {
    let code: String = String::new();
    for (ln, line_result) in BufReader::new(File::open(file).unwrap()).lines().enumerate() {
        let line = line_result.unwrap().clone();
        let _tokenizer = Tonkenizer::new(&line, Loc::new(file, ln, 0));
        while let Some(data) = _tokenizer.next() {
            println!("{:?}", data);
        }
        
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("{}", code);
}
