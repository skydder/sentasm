use std::cell::RefCell;
use crate::PUNCTUATOR;

fn is_punctuator(s: &str)  -> bool {
    for punctuator in PUNCTUATOR {
        if s.starts_with(punctuator) {
            return true;
        }
    }
    false
}

#[derive(Debug)]
pub struct StreamInfo<'a> {
    file: &'a str,
    length: usize,
}

pub struct Stream<'a> {
    pub stream: &'a str,
    stream_info: StreamInfo<'a>
}

#[derive(Debug)]
pub struct Location<'a> {
    stream_info: &'a StreamInfo<'a>,
    nth: usize,
    line: usize,
    column: usize
}

impl<'a> Location<'a> {
    fn get_nth(&self) -> usize {
        self.nth
    }
}

#[derive(Debug)]
pub enum Token<'a> {
    Identifier(&'a str, Location<'a>),
    Number(i64, Location<'a>),
    Punctuator(&'a str, Location<'a>),
    EOL,
    EOF,
}

pub struct Tokenizer<'a> {
    stream: &'a Stream<'a>,
    next_location: RefCell<Location<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(stream: &'a Stream<'a>) -> Self {
        let loc = Location { stream_info: &stream.stream_info, line: 1, column: 1 , nth: 0};
        Self { stream: stream, next_location: RefCell::new(loc) }
    }

    pub fn next(&self) -> Token {
        match self.peek() {
            Token::Identifier(ident, loc) => {
                self.slide_location_by(ident.len());
                Token::Identifier(ident, loc)
            },
            Token::Punctuator(punc, loc ) => {
                self.slide_location_by(punc.len());
                Token::Punctuator(punc, loc)
            },
            Token::Number(num, loc) => {
                self.slide_location_by(self.find_next_punctuator_or_whitespace() - loc.nth);
                Token::Number(num, loc)
            },
            token => token
        }
    }

    pub fn peek(&self) -> Token {
        if let Some(token) = self.consume_whitespaces() {
            token
        } else if let Some(token) = self.peek_punctuator() {
            token
        } else if let Some(token) = self.peek_number() {
            token
        } else {
            Token::Identifier(self.slice_stream(self.get_nth(), self.find_next_punctuator_or_whitespace()), self.get_location())
        }
    }

    fn get_nth_letter_of_stream(&self, nth: usize) -> Option<char> {
        self.stream.stream.chars().nth(nth)
    }

    fn slice_stream(&self, start: usize, end: usize) -> &str {
        &self.stream.stream[start..end]
    }

    fn slice_stream_from_nth(&self, nth: usize) -> &str {
        &self.stream.stream[nth..]
    }
    fn slide_location_by(&self, n: usize) {
        self.next_location.borrow_mut().nth += n;
        self.next_location.borrow_mut().column += n;
    }

    fn consume_whitespaces(&self) -> Option<Token>{
        while self.get_nth_letter_of_stream(self.get_nth()).is_some_and(|c| c.is_whitespace()) {
            let ws = self.get_nth_letter_of_stream(self.get_nth()).unwrap();
            self.slide_location_by(1);
            if ws == '\n' {
                self.next_location.borrow_mut().line += 1;
                return Some(Token::EOL);
            }
        }
        if self.get_nth_letter_of_stream(self.get_nth()).is_none() {
            return Some(Token::EOF);
        }
        None
    }

    fn get_nth(&self) -> usize{
        self.next_location.borrow().get_nth()
    }

    fn get_location(&self) -> Location<'a> {
        Location { stream_info: &self.stream.stream_info, nth: self.get_nth(), line: self.next_location.borrow().line, column: self.next_location.borrow().column }
    }

    fn peek_punctuator(&self) -> Option<Token> {
        for punctuator in PUNCTUATOR {
            if self.stream.stream[self.get_nth()..].starts_with(punctuator) {
                let location = self.get_location();
                return Some(Token::Punctuator(&punctuator, location));
            }
        }
        None
    }
    fn find_next_punctuator_or_whitespace(&self) -> usize {
        let mut nth = self.get_nth(); 
        while self.get_nth_letter_of_stream(nth).is_some_and(|c| !c.is_whitespace()) && !is_punctuator(self.slice_stream_from_nth(nth)){
            nth += 1;
        }
        nth
    }
    fn peek_number(&self) -> Option<Token> {
        if self.get_nth_letter_of_stream(self.get_nth()).is_some_and(|c| c.is_ascii_hexdigit()) {
            if let Ok(number) = self.slice_stream(self.get_nth(), self.find_next_punctuator_or_whitespace()).parse::<i64>() {
                return Some(Token::Number(number, self.get_location()));
            }
        }
        None
    }
}

#[test]
fn test() {
    let stream = &Stream { stream: "substract 1 from @[ax+rax*1](base+idx*scl)", stream_info: StreamInfo { file: "test", length: 100 } };
    let tokenizer = Tokenizer::new(stream);
    loop {
        match tokenizer.next() {
            Token::EOF => break,
            token => {
                println!("{:?}", token)
            }       
        };
    }
}