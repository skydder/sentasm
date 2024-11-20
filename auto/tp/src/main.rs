use std::{fs::{self, File}, io::Read};

fn main() {
    let file = open_file("grammar.dat");
    print(file);
}

fn open_file(file: &str) -> File {
    File::open(file).unwrap_or_else(|_| {
        eprintln!("failed to open '{}'", file);
        ::std::process::exit(1);
    })
}

fn print(mut file: File) {
    let mut s = String::new();
    let _ = file.read_to_string(&mut s);
    println!("{}", s);
}
