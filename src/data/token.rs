pub struct Token<'a> {
    pub seq: &'a str,
    location: usize,
    pub len: usize,
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

    pub fn inspect(&mut self) -> &'a str {
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

    pub fn next(&mut self) {
        self.location += self.len;
        while self
            .seq
            .chars()
            .nth(self.location)
            .unwrap_or('a')
            .is_whitespace()
        {
            self.location += 1;
        }
        self.len = self.calculate_len();
    }

    pub fn location(&self) -> usize {
        self.location
    }
}
