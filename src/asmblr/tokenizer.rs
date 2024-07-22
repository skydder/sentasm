use std::cell::Cell;

use super::{Data, DataSet};

#[derive(Clone, Copy)]
pub struct Loc<'a> {
    file_name: &'a str,
    line: usize,
    column: usize,
}

impl<'a> Loc<'a> {
    pub fn new(file_name:&'a str, line: usize, column: usize) -> Self{
        Self { file_name, line, column}
    }
}

pub struct Tonkenizer<'a> {
    sourse: &'a str,
    loc: Cell<Loc<'a>>,
}

impl<'a> Tonkenizer<'a> {
    pub fn new(sourse: &'a str, loc: Loc<'a>) -> Self {
        Self {
            sourse,
            loc: Cell::new(loc)
        }
    }

    fn skip_whitespase(&self) {
        let mut loc = self.loc.get();
        while self.sourse.chars().nth(loc.column).unwrap_or('*').is_whitespace() {

            loc.column += 1;
        }
        self.loc.set(loc)
    }

    fn length_of_symbol(&self) -> usize {
        let mut len = 0;
        let column = self.loc.get().column;
        let src = self.sourse.replace('[', " [ ").replace(']', " ] ").replace('@', " @ ").replace(':', " : ");
        while !src.chars().nth(column + len).unwrap_or('\n').is_whitespace() {
            len += 1;
        }
        len 
    }

    pub fn peek(&self) -> Option<DataSet> {
        self.skip_whitespase();
        let column = self.loc.get().column;
        match self.length_of_symbol() {
            0 => None,
            _ => Some(DataSet::new(&self.sourse[column..column + self.length_of_symbol()], self.loc.get()))
        }
    }

    pub fn next(&self) -> Option<DataSet> {
        let next = self.peek();
        let column = self.loc.get().column + self.length_of_symbol();
        self.loc.set(Loc { file_name: self.loc.get().file_name, line: self.loc.get().line, column: column });
        next
    }
    
}

