use crate::PUNCTUATOR;
use std::cell::RefCell;
use crate::emit_error;

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

impl<'a> std::fmt::Display for Location<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.stream_info.file, self.line, self.column)
    }
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

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Identifier(&'a str, Location<'a>),
    Number(i64, Location<'a>),
    Punctuator(&'a str, Location<'a>), 
    String(&'a str, Location<'a>),
    RawNasm(&'a str, Location<'a>),
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

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::EOF => write!(f, "EOF"),
            Token::EOL => write!(f, "\\n"),
            Token::Identifier(ident, ..) => write!(f, "{}", ident),
            Token::Punctuator(punc, ..) => write!(f, "{}", punc),
            Token::String(s, ..) => write!(f, "\"{}\"", s),
            Token::Number(num, ..) => write!(f, "{}", num),
            Token::RawNasm(nasm, ..) => write!(f, "{}", nasm),
        }
    }
}

pub struct Tokenizer<'a> {
    stream: &'a Stream<'a>,
    next_location: RefCell<Location<'a>>,
    peek: RefCell<((Token<'a>, Location<'a>, usize), bool)>,
    peek2: RefCell<((Token<'a>, Location<'a>, usize), bool)>,
    peek3: RefCell<((Token<'a>, Location<'a>, usize), bool)>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(stream: &'a Stream<'a>) -> Self {
        let loc = Location {
            stream_info: &stream.stream_info,
            line: 1,
            column: 1,
            nth: 0,
        };
        let new_one = Self {
            stream: stream,
            next_location: RefCell::new(loc),
            peek: RefCell::new(((Token::EOL, loc, 0), false)),
            peek2: RefCell::new(((Token::EOL, loc, 0), false)),
            peek3: RefCell::new(((Token::EOL, loc, 0), false)),
        };
        new_one.peek();
        new_one.peek2();
        new_one.peek3();
        new_one
    }

    fn peek_at(&self, location: Location<'a>) -> (Token<'a>, Location<'a>, usize) {
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
        } else if let Some((token, len)) = self.peek_raw_nasm_of(cur_loc) {
            (token, cur_loc, len)
        }else {
            let start = cur_loc.get_nth();
            let end = self.find_next_punctuator_or_whitespace_from(&cur_loc);
            (Token::Identifier(
                self.slice_stream(start, end),
                cur_loc, 
            ), cur_loc, end - start)
        }
    }

    pub fn peek(&self) -> Token<'a> {
        if self.peek.borrow().1 {
            self.peek.borrow().0.0
        } else {
            let peeked = self.peek_at(self.get_location());
            self.peek.replace((peeked, true));
            self.set_next_location(peeked.1);
            peeked.0
        }
    }

    pub fn peek2(&self) -> Token<'a> {
        if self.peek2.borrow().1 {
            self.peek2.borrow().0.0
        } else {
            self.peek();
            let (token, loc, len) = (self.peek.borrow().0.0, self.peek.borrow().0.1, self.peek.borrow().0.2);
            let peeked = match token {
                Token::EOF => (token, loc, len),
                Token::EOL => self.peek_at(loc.slide_by(len).next_line()),
                _ => self.peek_at(loc.slide_by(len))
            };
            
            self.peek2.replace((peeked, true));
            peeked.0
        }
    }
    pub fn peek3(&self) -> Token<'a> {
        if self.peek3.borrow().1 {
            self.peek3.borrow().0.0
        } else {
            self.peek2();
            let (token, loc, len) = (self.peek2.borrow().0.0, self.peek2.borrow().0.1, self.peek2.borrow().0.2);
            let peeked = match token {
                Token::EOF => (token, loc, len),
                Token::EOL => self.peek_at(loc.slide_by(len).next_line()),
                _ => self.peek_at(loc.slide_by(len))
            };
            
            self.peek3.replace((peeked, true));
            peeked.0
        }
    }

    pub fn next(&self) -> Token<'a> {
        let (token, loc, len) = (self.peek.borrow().0.0, self.peek.borrow().0.1, self.peek.borrow().0.2);
        self.set_next_location(loc);
        self.slide_location_by(len);
        if let Token::EOL = token {
            let loc = self.get_location().next_line();
            self.set_next_location(loc);
        }
        self.peek.replace(((self.peek2.borrow().0.0, self.peek2.borrow().0.1, self.peek2.borrow().0.2), true));
        self.peek2.replace(((self.peek3.borrow().0.0, self.peek3.borrow().0.1, self.peek3.borrow().0.2), true));
        self.peek3.borrow_mut().1 = false;
        self.peek3();
        token
    }

    pub fn expect_punctuator(&self, punc: &'a str) {
        if self.peek().get_punctuator().is_some_and(|p| p == punc) {
            self.next();
        } else {
            // error
            emit_error!(self.get_location(), "expected '{}', but found '{}'.", punc, self.peek());
        }
    }

    pub fn expect_end_of_line(&self) {
        if let Token::EOL = self.peek() {
            self.next();
        } else if let Token::EOF = self.peek() {
            self.next();
        } else {
            // error
            emit_error!(self.get_location(), "expected '\\n', but found '{}'.", self.peek());
        }
    }

    pub fn is_end_of_line(&self) -> bool {
        if let Token::EOL = self.peek() {
            true
        } else {
            false
        }
    } 
    pub fn is_eof(&self) -> bool {
        if let Token::EOF = self.peek() {
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

    fn slice_stream(&self, start: usize, end: usize) -> &'a str {
        &self.stream.stream[start..end]
    }

    fn slice_stream_from_nth(&self, nth: usize) -> &'a str {
        &self.stream.stream[nth..]
    }

    fn slide_location_by(&self, n: usize) {
        let loc = self.get_location().slide_by(n);
        self.set_next_location(loc);
    }

    fn set_next_location(&self, location: Location<'a>) {
        self.next_location.replace(location);
    }

    fn consume_whitespaces_of(&self, location: Location<'a>) -> (Option<Token<'a>>, Location<'a>) {
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
        self.skip_comment(loc)
    }

    fn skip_comment(&self, location: Location<'a>) -> (Option<Token<'a>>, Location<'a>) {
        let mut loc = location;
        if !self.get_nth_letter_of_stream(loc.get_nth()).is_some_and(|c| c == '(') {
            return (None, loc);
        } else {
            loc = loc.slide_by(1);
        }
        while self
            .get_nth_letter_of_stream(loc.get_nth())
            .is_some_and(|c| !(c == ')' || c == '\n'))
        {
            loc = loc.slide_by(1);
        }
        if self.get_nth_letter_of_stream(loc.get_nth()).is_none() {
            return (Some(Token::EOF), loc);
        } else if self.get_nth_letter_of_stream(loc.get_nth()).is_some_and(|c| c == '\n') {
            return (Some(Token::EOL), loc);
        }
        loc = loc.slide_by(1);
        self.consume_whitespaces_of(loc)
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

    fn peek_punctuator_of(&self, location: Location<'a>) -> Option<(Token<'a>, usize)> {
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

    fn peek_number_of(&self, location: Location<'a>) -> Option<(Token<'a>, usize)> {
        if self
            .get_nth_letter_of_stream(location.get_nth())
            .is_some_and(|c| c.is_ascii_hexdigit())
        {
            let end = self.find_next_punctuator_or_whitespace_from(&location);
            if let Ok(number) = self
                .slice_stream(location.get_nth(), end)
                .parse::<i64>()
            {
                let start = location.get_nth();
                return Some((Token::Number(number, location), end - start));
            }
        }
        None
    }

    fn peek_string_of(&self, location: Location<'a>) -> Option<(Token<'a>, usize)> {
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
                emit_error!(location, "string-literal cannot be over the line.");
            }
            Some((Token::String(self.slice_stream(start + 1, nth), location), (nth - start + 1)))
        } else {
            None
        }
        
    }

    fn peek_raw_nasm_of(&self, location: Location<'a>) -> Option<(Token<'a>, usize)> {
        let start = location.get_nth();
        if self
            .get_nth_letter_of_stream(start)
            .is_some_and(|c| c == ';')
        {
            let mut nth = start + 1;
            while self
                .get_nth_letter_of_stream(nth)
                .is_some_and(|c| c != '\n')
            {
                nth += 1;
            }
            Some((Token::RawNasm(self.slice_stream(start + 1, nth), location), nth - start))
        } else {
            None
        }
        
    }
}

#[test]

fn test() {
    let stream = Stream::new("substract 1 from @[ax+rax*1]\n(base+idx*scl)\n \"move test\" substract\n!nasm test\n", "test");
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
