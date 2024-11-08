#[derive(Clone, Copy, Debug)]
pub struct Keyword<'a>(pub &'a str);

impl<'a> std::fmt::Display for Keyword<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword("double-precision-float") => write!(f, "sd"),
            Keyword("single-precision-float") => write!(f, "ss"),
            Keyword("==") => write!(f, "e"),
            Keyword(">") => write!(f, "g"),
            Keyword("<") => write!(f, "l"),
            Keyword("!=") => write!(f, "ne"),
            Keyword(">=") => write!(f, "ge"),
            Keyword("<=") => write!(f, "le"),
            Keyword("8bit") => write!(f, "b"),
            Keyword("16bit") => write!(f, "w"),
            Keyword("32bit") => write!(f, "d"),
            Keyword("64bit") => write!(f, "q"),
            Keyword("signed") => write!(f, ""),
            _ => write!(f, "**"),
        }
    }
}
