use std::cell::RefCell;

pub struct StreamInfo<'a> {
    file: &'a str,
    length: usize,
}

pub struct Stream<'a> {
    stream: &'a str,
    stream_info: StreamInfo<'a>
}

pub struct Location<'a> {
    stream_info: &'a StreamInfo<'a>,
    line: usize,
    column: usize
}

pub enum Token<'a> {
    Identifier(&'a str, Location<'a>),
    Number(i64, Location<'a>),
    Punctuator(&'a str, Location<'a>),
}

pub struct Tokenizer<'a> {
    stream: &'a Stream<'a>,
    next_location: RefCell<Location<'a>>
}

impl<'a> Tokenizer<'a> {
    pub fn new(stream: &'a Stream<'a>) -> Self {
        let loc = Location { stream_info: &stream.stream_info, line: 0, column: 0 };
        Self { stream: stream, next_location: RefCell::new(loc) }
    }

    pub fn next(&self) -> Token {
        todo!()
    }

    pub fn peek(&self) -> Token {
        todo!()
    }
}