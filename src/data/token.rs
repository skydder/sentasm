use core::fmt;

#[derive(Debug)]
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
        write!(f,"")
    }
}
#[derive(Debug)]
pub struct Token<'a> {
    pub(crate) seq: &'a str,
    pub(crate) location: TokenLocation<'a>,
    pub(crate) len: usize,
}

impl<'a> Token<'a> {
    pub fn tokenize(s: &'a str, location:TokenLocation) -> Self {
        let mut new = Self {
            seq: s,
            location: location,
            len: 0,
        };

        new.len = new.calculate_len();
        new
    }

    pub(crate) fn _inspect(&self) -> &'a str {
        return &self.seq[self.location.column..self.location.column + self.len];
    }

    fn calculate_len(&self) -> usize {
        let mut len = 0;
        while !self
            .seq
            .chars()
            .nth(self.location.column + len)
            .unwrap_or(' ')
            .is_whitespace()
        {
            len += 1;
        }
        len
    }

    pub(crate) fn next(&mut self) {
        self.location.column += self.len;
        while self
            .seq
            .chars()
            .nth(self.location.column)
            .unwrap_or('a')
            .is_whitespace()
        {
            if self.seq.chars().nth(self.location.column).unwrap_or(' ') == '[' {
                while self.seq.chars().nth(self.location.column).unwrap_or(']') == ']' {}
                self.location.column += 1;
                break;
            }
            self.location.column += 1;
        }
        self.len = self.calculate_len();
        // println!("{:?}", self)
    }

    pub fn is_end(&self) -> bool {
        self.len == 0
    }
}
