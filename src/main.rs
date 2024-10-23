use std::{
    fs, process::exit
};

use codegen::codegen;
use tokenizer::{Tokenizer, Stream};
use parser::parser;

fn main() {
    match read_args() {
        Ok(file) => compile_file2(&file),
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

// fn compile_file(file: &str) {
//     let mut code = String::new();
//     for (ln, line_result) in BufReader::new(File::open(file).unwrap())
//         .lines()
//         .enumerate()
//     {
//         let line = line_result.unwrap().clone();
//         let _tokenizer = Tonkenizer::new(&line, Loc::new(file, ln, 0));
//         // println!("{:?}", Code::parse(&_tokenizer))
//         codegen(Code::parse(&_tokenizer).unwrap_or_else(|_| exit(404)), &mut code)
//         .unwrap_or_else(|_| exit(404));
//     }
//     println!("{}", code)
// }

fn compile_file2(file: &str) {
    let _stream = fs::read_to_string(file).expect("file loading error");
    let stream = Stream::new(&_stream, file);
    let tokenizer = Tokenizer::new(&stream);
    let mut code = String::new();
    while !tokenizer.is_eof() {
        codegen(parser(&tokenizer), &mut code).unwrap_or_else(|_| exit(404));
        // println!("{:#?}", parser(&tokenizer));
    }
    println!("{}", code)
}