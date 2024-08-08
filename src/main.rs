use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

mod asmblr;

use asmblr::{codegen, Code, Loc, Tonkenizer};

fn main() {
    match read_args() {
        Ok(file) => compile_file(&file),
        Err(_) => (),
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

fn compile_file(file: &str) {
    println!(".intel_syntax noprefix");
    println!(".global main");

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
