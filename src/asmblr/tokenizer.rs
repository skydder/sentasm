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

        if self.sourse.chars().nth(loc.column).unwrap_or('*') == '(' {
            while self.sourse.chars().nth(loc.column).unwrap_or(')') != ')' {
                loc.column += 1;
            }
        }
        self.loc.set(loc)
    }

    fn length_of_symbol(&self) -> usize {
        let mut len = 0;
        let column = self.loc.get().column;
        if self.sourse[column..].starts_with("@[") {
            while self.sourse.chars().nth(column + len).unwrap_or(']') != ']' {
                len += 1;
            }
            return len + 1;
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
        // println!("{}", &self.sourse[column..column + self.length_of_symbol()]);
        match self.length_of_symbol() {
            0 => None,
            _ => Some(DataSet::new(
                &self.sourse[column..column + self.length_of_symbol()],
                self.loc.get(),
            )),
        }
    }

    // pub fn peek_next(&self) -> Option<DataSet> {
    //     self.skip_whitespase();
    //     let column = self.loc.get().column;
    //     match self.length_of_symbol() {
    //         0 => return None,
    //         _ => ()
    //     };
    //     let toknizer = Tonkenizer::new(&self.sourse, Loc::new(self.loc.get().file_name, self.loc.get().line, column + self.length_of_symbol()));
    //     let data = toknizer.peek();
    //     data
    // }

    pub fn next(&self) -> Option<DataSet> {
        let next = self.peek();
        let column = self.loc.get().column + self.length_of_symbol();
        self.loc.set(Loc::new(
            self.loc.get().file_name,
            self.loc.get().line,
            column,
        ));
        next
    }
}
