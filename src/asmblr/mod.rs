mod tokenizer;
mod parser;
mod data;
pub use tokenizer::{Tonkenizer, Loc};
pub use data::{Data, DataSet};
use data::{Verb, Preposition, Keyword, Label};
use parser::Code;