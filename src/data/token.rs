#[derive(Debug)]
pub struct Token<'a> {
    pub(crate) seq: &'a str,
    location: usize,
    pub(crate) len: usize,
}

impl<'a> Token<'a> {
    pub fn tokenize(s: &'a str) -> Self {
        let mut new = Self {
            seq: s,
            location: 0,
            len: 0,
        };

        new.len = new.calculate_len();
        new
    }

    pub(crate) fn _inspect(&self) -> &'a str {
        return &self.seq[self.location..self.location + self.len];
    }

    fn calculate_len(&self) -> usize {
        let mut len = 0;
        while !self
            .seq
            .chars()
            .nth(self.location + len)
            .unwrap_or(' ')
            .is_whitespace()
        {
            len += 1;
        }
        len
    }

    pub(crate) fn next(&mut self) {
        self.location += self.len;
        while self
            .seq
            .chars()
            .nth(self.location)
            .unwrap_or('a')
            .is_whitespace()
        {
            if self.seq.chars().nth(self.location).unwrap_or(' ') == '[' {
                while self.seq.chars().nth(self.location).unwrap_or(']') == ']' {}
                self.location += 1;
                break;
            }
            self.location += 1;
        }
        self.len = self.calculate_len();
        println!("{:?}", self)
    }

    pub fn is_end(&self) -> bool {
        self.len == 0
    }
}
