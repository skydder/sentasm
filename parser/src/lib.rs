mod parse_register;
mod parse_memory;
mod parse_immediate;
mod parse_verb;
mod parse_preposition;
mod parse_keyword;


mod parser;

pub(crate) use parse_register::parse_register;
pub(crate) use parse_immediate::{parse_immediate, parse_number};
pub(crate) use parse_verb::parse_verb;
pub(crate) use parse_preposition::parse_preposition;
pub(crate) use parse_keyword::parse_keyword;
pub use parser::{parser, parse_data_set};
