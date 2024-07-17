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

pub enum Data {
    Verb,
    Register,
    Prepositon,
    Immidiate,
    MemoryLP,
    MemoryRP,
    MemorySym,
    Keyword,
}

pub struct DataSet<'a> {
    pub data:Data,
    pub loc: Loc<'a>,
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
        while self.sourse.chars().nth(self.column).unwrap_or('\n').is_whitespace() {
            self.column += 1;
        }
    }

    fn length_of_symbol(&self) -> usize {
        let mut len = 0;
        while !self.sourse.chars().nth(self.column).unwrap_or('\n').is_whitespace() {
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

const PREPOSITIONS: &[&'static str] = &["to", "by", "with", "from", "as", "if"];

impl<'a> DataSet<'a> {
    pub fn new(token: &'a str, loc: Loc<'a>) -> Self {
        Self{ data: Self::parse(token), loc}
    }

    fn parse(token: &'a str) -> Data {
        if token == "[" {
            Data::MemoryLP
        } else if token == "[" {
            Data::MemoryRP
        } else if Self::is_verb(token) {
            Data::Verb
        } else if Self::is_register(token) {
            Data::Register
        } else if let i = token.parse::<i64>() {
            Data::Immidiate
        } else if Self::is_keyword(token) {
            Data::Keyword
        } else if Self::is_preposition(token) {
            Data::Prepositon
        } else {
            panic!();
        }
    }

    fn is_verb(token: &'a str) -> bool {todo!()}
    fn is_register(token: &'a str) -> bool {todo!()}
    fn is_keyword(token: &'a str) -> bool {todo!()}
    fn is_preposition(token: &'a str) -> bool {
        for i in PREPOSITIONS {
            if token == *i {
                return true;
            }
        }
        return  false;
    }
}