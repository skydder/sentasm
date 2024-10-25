// later to implement float
pub enum DefItem<'a> {
    Int(i64),
    Str(&'a str),
}

impl<'a> std::fmt::Debug for DefItem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DefItem::Int(i) => write!(f, "{}", i),
            DefItem::Str(s) => write!(f, "\"{}\"", s),
        }
    }
}

#[derive(Debug)]
pub struct Define<'a> {
    list: Vec<DefItem<'a>>,
}

impl<'a> Define<'a> {
    pub fn new(list: Vec<DefItem<'a>>) -> Self {
        Self { list: list }
    }

}

impl<'a> std::fmt::Display for Define<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pos = 1;
        write!(f, "{:?}", self.list[0])?;
        while pos < self.list.len() {
            write!(f, ", {:?}", self.list[pos])?;
            pos += 1;
        }
        Ok(())
    }
}
