use core::{fmt, cell::RefCell};

#[derive(Debug, Clone, Copy)]
pub struct TokenLocation<'a> {
    flie_name: &'a str,
    line: usize,
    column: usize,
}

impl<'a> TokenLocation<'a> {
    pub fn new(file_name:&'a str,line: usize, column: usize) -> Self {
        Self { line: line, column: column, flie_name: file_name }
    }
}

impl<'a> fmt::Display for TokenLocation<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}({}:{})", self.flie_name, self.line, self.column)
    }
}
#[derive(Debug)]
pub struct Token<'a> {
    pub(crate) seq: &'a str,
    pub(crate) location: RefCell<TokenLocation<'a>>,
    pub(crate) len: RefCell<usize>,
}

impl<'a> Token<'a> {
    pub fn tokenize(s: &'a str, location:TokenLocation<'a>) -> Self {
        let new = Self {
            seq: s,
            location: RefCell::new(location),
            len: RefCell::new(0),
        };

        *new.len.borrow_mut() = new.calculate_len();
        new
    }

    pub(crate) fn _inspect(&self) -> &'a str {
        return &self.seq[self.location.borrow().column..self.location.borrow().column + *self.len.borrow()];
    }

    fn calculate_len(&self) -> usize {
        let mut len = 0;
        while !self
            .seq
            .chars()
            .nth(self.location.borrow().column + len)
            .unwrap_or(' ')
            .is_whitespace()
        {
            len += 1;
        }
        len
    }

    pub(crate) fn next(&self) {
        self.location.borrow_mut().column += *self.len.borrow();
        while self
            .seq
            .chars()
            .nth(self.location.borrow().column)
            .unwrap_or('a')
            .is_whitespace()
        {
            if self.seq.chars().nth(self.location.borrow().column).unwrap_or(' ') == '[' {
                while self.seq.chars().nth(self.location.borrow().column).unwrap_or(']') == ']' {}
                self.location.borrow_mut().column += 1;
                break;
            }
            self.location.borrow_mut().column += 1;
        }
        *self.len.borrow_mut() = self.calculate_len();
        // println!("{:?}", self)
    }

    pub fn is_end(&self) -> bool {
        *self.len.borrow() == 0
    }
}
