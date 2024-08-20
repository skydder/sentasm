use std::cell::Cell;

use super::DataSet;

#[derive(Clone, Copy, Debug)]
pub struct Loc<'a> {
    file_name: &'a str,
    line: usize,
    column: usize,
}

impl<'a> Loc<'a> {
    pub fn new(file_name: &'a str, line: usize, column: usize) -> Self {
        Self {
            file_name,
            line,
            column,
        }
    }
}

impl<'a> std::fmt::Display for Loc<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n->{}:{}:{}", self.file_name, self.line + 1, self.column + 1)
    }
}

#[derive(Debug)]
pub struct Tonkenizer<'a> {
    sourse: String,
    loc: Cell<Loc<'a>>,
}

impl<'a> Tonkenizer<'a> {
    pub fn new(sourse: &'a str, loc: Loc<'a>) -> Self {
        Self {
            sourse: sourse.replace('#', " # ").replace(':', " : "),
            loc: Cell::new(loc),
        }
    }

    fn skip_whitespase(&self) {
        let mut loc = self.loc.get();
        while self
            .sourse
            .chars()
            .nth(loc.column)
            .unwrap_or('*')
            .is_whitespace()
        {
            loc.column += 1;
        }
        // skip comments
        if self.sourse.chars().nth(loc.column).unwrap_or('*') == '(' {
            while self.sourse.chars().nth(loc.column).unwrap_or(')') != ')' {
                loc.column += 1;
            }
            if self.sourse.chars().nth(loc.column).is_some() {
                loc.column += 1;
            }
        }
        self.loc.set(loc)
    }

    fn length_of_symbol(&self) -> usize {
        let mut len = 0;
        let column = self.loc.get().column;

        // memory, section, label, define
        if self.sourse[column..].starts_with("@[") || self.sourse[column..].starts_with("[") {
            while self.sourse.chars().nth(column + len).unwrap_or(']') != ']' {
                len += 1;
            }
            return len + 1;
        } else if  self.sourse[column..].starts_with("@") | self.sourse[column..].starts_with("#") {
            return 1;
        }
        while !self
            .sourse
            .chars()
            .nth(column + len)
            .unwrap_or('\n')
            .is_whitespace()
        {
            len += 1;
        }
        len
    }

    pub fn peek(&self) -> Option<DataSet> {
        self.skip_whitespase();
        let column = self.loc.get().column;
        
        match self.length_of_symbol() {
            0 => None,
            _ => Some(DataSet::new(
                &self.sourse[column..column + self.length_of_symbol()],
                self.loc.get(),
            )),
        }
    }

    pub fn next(&self) -> Option<DataSet> {
        let next = self.peek();
        // println!("{:?}", next);
        let column = self.loc.get().column + self.length_of_symbol();
        self.loc.set(Loc::new(
            self.loc.get().file_name,
            self.loc.get().line,
            column,
        ));
        next
    }

    pub fn loc(&self) -> Loc<'a> {
        self.loc.get()
    }
}

impl<'a> std::fmt::Display for Tonkenizer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.sourse[self.loc().column..self.loc().column + self.length_of_symbol()])
    }
}
