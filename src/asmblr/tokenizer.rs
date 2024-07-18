use super::{Data, DataSet};

pub struct Tonkenizer<'a> {
    sourse: &'a str,
    file_name: &'a str,
    line: usize,
    column: usize,
}

pub struct Loc<'a> {
    file_name: &'a str,
    line: usize,
    column: usize,
}


impl<'a> Tonkenizer<'a> {
    pub fn new(sourse: &'a str, loc: Loc<'a>) -> Self {
        Self {
            sourse,
            file_name: loc.file_name,
            line: loc.line,
            column: loc.column
        }
    }

    pub fn location(&self) -> Loc<'a> {
        Loc { file_name: self.file_name, line: self.line, column: self.column }
    }

    fn skip_whitespase(&mut self) {
        while self.sourse.chars().nth(self.column).unwrap_or('*').is_whitespace() {
            self.column += 1;
        }
    }

    fn length_of_symbol(&self) -> usize {
        let mut len = 0;
        let src = self.sourse.replace('[', " [ ").replace(']', " ] ").replace('@', " @ ").replace(':', " : ");
        while !src.chars().nth(self.column + len).unwrap_or('\n').is_whitespace() {
            len += 1;
        }
        len 
    }

    pub fn next_symbol(&mut self) -> Option<DataSet> {
        self.skip_whitespase();
        match self.length_of_symbol() {
            0 => None,
            _ => Some(DataSet::new(&self.sourse[self.column..self.column + self.length_of_symbol()], self.location()))
        }
    }
}

impl<'a> Loc<'a> {
    pub fn new(file_name:&'a str, line: usize, column: usize) -> Self{
        Self { file_name, line, column}
    }
}