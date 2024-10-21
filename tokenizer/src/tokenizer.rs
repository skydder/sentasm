use crate::PUNCTUATOR;
use std::cell::RefCell;

fn is_punctuator(s: &str) -> bool {
    for punctuator in PUNCTUATOR {
        if s.starts_with(punctuator) {
            return true;
        }
    }
    false
}

#[derive(Debug)]
pub struct StreamInfo<'a> {
    pub file: &'a str,
    pub length: usize,
}

impl<'a> StreamInfo<'a> {
    pub fn get_file(&self) -> &str {
        self.file
    }
    pub fn get_length(&self) -> usize {
        self.length
    }
}

impl<'a> Default for StreamInfo<'a> {
    fn default() -> Self {
        Self { file: "", length: 0 }
    }
}

pub struct Stream<'a> {
    pub stream: &'a str,
    stream_info: StreamInfo<'a>,
}

impl<'a> Stream<'a> {
    pub fn new(stream: &'a str, file: &'a str) -> Self {
        Self {
            stream: stream,
            stream_info: StreamInfo {
                file: file,
                length: stream.len(),
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Location<'a> {
    stream_info: &'a StreamInfo<'a>,
    nth: usize,
    line: usize,
    column: usize,
}

impl<'a> Location<'a> {
    pub fn new(stream_info: &'a StreamInfo<'a>) -> Self {
        Self { stream_info: stream_info, nth: 0, line: 1, column: 1 }
    }
    pub fn get_nth(&self) -> usize {
        self.nth
    }
    pub fn get_stream_info(&self) -> &StreamInfo<'a> {
        self.stream_info
    }
    pub fn slide_by(&self, n: usize) -> Self {
        Self { stream_info: self.stream_info, nth: self.nth + n, line: self.line, column: self.column + n }
    }

    pub fn next_line(&self) -> Self {
        Self { stream_info: self.stream_info, nth: self.nth, line: self.line + 1, column: 1 }
    }
}

#[derive(Debug)]
pub enum Token<'a> {
    Identifier(&'a str, Location<'a>),
    Number(i64, Location<'a>),
    Punctuator(&'a str, Location<'a>), 
    String(&'a str, Location<'a>),
    EOL,
    EOF,
}

impl<'a> Token<'a> {
    pub fn is_identifier(&self) -> bool {
        match self {
            Token::Identifier(..) => true,
            _ => false
        }
    }

    pub fn get_identifier(&self) -> Option<&str> {
        match self {
            Token::Identifier(ident,_) => Some(ident),
            _ => None
        }
    }

    pub fn get_punctuator(&self) -> Option<&str> {
        match self {
            Token::Punctuator(punc,_) => Some(punc),
            _ => None
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Token::Number(..) => true,
            _ => false
        }
    }

    pub fn is_punctuator(&self) -> bool {
        match self {
            Token::Punctuator(..) => true,
            _ => false
        }
    }
}

pub struct Tokenizer<'a> {
    stream: &'a Stream<'a>,
    next_location: RefCell<Location<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(stream: &'a Stream<'a>) -> Self {
        let loc = Location {
            stream_info: &stream.stream_info,
            line: 1,
            column: 1,
            nth: 0,
        };
        Self {
            stream: stream,
            next_location: RefCell::new(loc),
        }
    }

    fn peek_at(&self, location: Location<'a>) -> (Token, Location<'a>, usize) {
        let mut cur_loc = location;
        let (tok, loc) = self.consume_whitespaces_of(cur_loc);
        cur_loc = loc;
        if let Some(token) = tok {
            (token, cur_loc, 1)
        } else if let Some((token,len)) = self.peek_punctuator_of(cur_loc) {
            (token, cur_loc, len)
        } else if let Some((token, len)) = self.peek_number_of(cur_loc) {
            (token, cur_loc, len)
        } else if let Some((token, len)) = self.peek_string_of(cur_loc) {
            (token, cur_loc, len)
        } else {
            let start = cur_loc.get_nth();
            let end = self.find_next_punctuator_or_whitespace_from(&cur_loc);
            (Token::Identifier(
                self.slice_stream(start, end),
                cur_loc, 
            ), cur_loc, end - start)
        }
    }

    pub fn peek(&self) -> Token {
        let (token, loc, _) = self.peek_at(self.get_location());
        self.set_next_location(loc);
        token
    }

    pub fn peek2(&self) -> Token {
        let (token, loc, len) = self.peek_at(self.get_location());
        self.set_next_location(loc);
        match token {
            Token::EOF => Token::EOF,
            Token::EOL => {
                let (token2, _, _) = self.peek_at(loc.slide_by(len).next_line());
                token2
            },
            _ => {
                let (token2, _, _) = self.peek_at(loc.slide_by(len));
                token2
            }           
        }
    }
    pub fn peek3(&self) -> Token {
        let (token, loc, len) = self.peek_at(self.get_location());
        self.set_next_location(loc);
        match token {
            Token::EOF => Token::EOF,
            Token::EOL => {
                let (_, loc2, len2) = self.peek_at(loc.slide_by(len).next_line());
                let (token3, _, _) = self.peek_at(loc2.slide_by(len2).next_line());
                token3
            },
            _ => {
                let (_, loc2, len2) = self.peek_at(loc.slide_by(len).next_line());
                let (token3, _, _) = self.peek_at(loc2.slide_by(len2).next_line());
                token3
            }           
        }
    }

    pub fn next(&self) -> Token {
        let (token, loc, len) = self.peek_at(self.get_location());
        self.set_next_location(loc);
        self.slide_location_by(len);
        if let Token::EOL = token {
            let loc = self.get_location().next_line();
            self.set_next_location(loc);
        }
        token
    }

    pub fn expect_punctuator(&self, punc: &'a str) {
        if self.peek().get_punctuator().is_some_and(|p| p == punc) {
            self.next();
        } else {
            // error
            eprintln!("{:?}", self.peek());
            todo!()
        }
    }

    pub fn expect_end_of_line(&self) {
        if let Token::EOL = self.peek() {
            self.next();
        } else if let Token::EOF = self.peek() {
            self.next();
        } else {
            eprintln!("{:?}", self.peek());
            // error
            todo!()
        }
    }

    pub fn is_end_of_line(&self) -> bool {
        if let Token::EOL = self.peek() {
            true
        } else {
            false
        }
    }

    pub fn is_number(&self) -> bool {
        match self.peek_number_of(self.get_location()) {
            Some(..) => true,
            None => false
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
        let loc = self.get_location().slide_by(n);
        self.set_next_location(loc);
    }

    fn set_next_location(&self, location: Location<'a>) {
        self.next_location.replace(location);
    }

    fn consume_whitespaces_of(&self, location: Location<'a>) -> (Option<Token>, Location<'a>) {
        let mut loc = location;
        while self
            .get_nth_letter_of_stream(loc.get_nth())
            .is_some_and(|c| c.is_whitespace())
        {
            let ws = self.get_nth_letter_of_stream(loc.get_nth()).unwrap();
            if ws == '\n' {
                return (Some(Token::EOL), loc);
            }
            loc = loc.slide_by(1);
        }
        if self.get_nth_letter_of_stream(loc.get_nth()).is_none() {
            return (Some(Token::EOF), loc);
        }
        (None, loc)
    }

    fn get_nth(&self) -> usize {
        self.next_location.borrow().get_nth()
    }

    pub fn get_location(&self) -> Location<'a> {
        Location {
            stream_info: &self.stream.stream_info,
            nth: self.get_nth(),
            line: self.next_location.borrow().line,
            column: self.next_location.borrow().column,
        }
    }

    fn peek_punctuator_of(&self, location: Location<'a>) -> Option<(Token, usize)> {
        for punctuator in PUNCTUATOR {
            if self.stream.stream[location.get_nth()..].starts_with(punctuator) {
                return Some((Token::Punctuator(&punctuator, location), punctuator.len()));
            }
        }
        None
    }

    fn find_next_punctuator_or_whitespace_from(&self, location: &Location<'a>) -> usize {
        let mut nth = location.get_nth();
        while self
            .get_nth_letter_of_stream(nth)
            .is_some_and(|c| !c.is_whitespace())
            && !is_punctuator(self.slice_stream_from_nth(nth))
        {
            nth += 1;
        }
        nth
    }

    fn peek_number_of(&self, location: Location<'a>) -> Option<(Token, usize)> {
        if self
            .get_nth_letter_of_stream(location.get_nth())
            .is_some_and(|c| c.is_ascii_hexdigit())
        {
            if let Ok(number) = self
                .slice_stream(location.get_nth(), self.find_next_punctuator_or_whitespace_from(&location))
                .parse::<i64>()
            {
                let start = location.get_nth();
                let end = self.find_next_punctuator_or_whitespace_from(&location);
                return Some((Token::Number(number, location), end - start));
            }
        }
        None
    }

    fn peek_string_of(&self, location: Location<'a>) -> Option<(Token, usize)> {
        let start = location.get_nth();
        if self
            .get_nth_letter_of_stream(start)
            .is_some_and(|c| c == '"')
        {
            let mut nth = start + 1;
            while !self
                .get_nth_letter_of_stream(nth)
                .is_some_and(|c| c == '"' || c == '\n')
            {
                nth += 1;
            }
            if self
                .get_nth_letter_of_stream(nth)
                .is_some_and(|c| c == '\n')
            {
                // error
                todo!()
            }
            Some((Token::String(self.slice_stream(start + 1, nth), location), (nth - start + 1)))
        } else {
            None
        }
        
    }
}

#[test]

fn test() {
    let stream = Stream::new("substract 1 from @[ax+rax*1]\n(base+idx*scl)\n \"move test\" substract", "test");
    let tokenizer = Tokenizer::new(&stream);
    eprintln!("peek2 : {:?}", tokenizer.peek2());
    loop {
        match tokenizer.next() {
            Token::EOF => break,
            token => {
                eprintln!("===");
                eprintln!("0: {:?}", token);
                eprintln!("1: {:?}", tokenizer.peek());
                eprintln!("2: {:?}", tokenizer.peek2());
                eprintln!("3: {:?}", tokenizer.peek3());
                eprintln!("===");
            }
        };
    }
}
