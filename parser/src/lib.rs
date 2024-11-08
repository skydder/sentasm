mod parse_code;
mod parse_define;
mod parse_immediate;
mod parse_keyword;
mod parse_memory;
mod parse_prep_phrases;
mod parse_preposition;
mod parse_register;
mod parse_verb;

mod parser;

pub(crate) use parse_code::parse_code;
pub(crate) use parse_define::parse_define;
pub(crate) use parse_immediate::{parse_immediate, parse_number};
pub(crate) use parse_keyword::parse_keyword;
pub(crate) use parse_memory::parse_memory;
pub(crate) use parse_prep_phrases::parse_prep_phrases;
pub(crate) use parse_preposition::parse_preposition;
pub(crate) use parse_register::parse_register;
pub(crate) use parse_verb::parse_verb;
pub(crate) use parser::parse_data_set;

pub use parser::parser;
