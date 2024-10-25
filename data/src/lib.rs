mod data_auto;
mod data;
mod code;
mod memory;
mod label;
mod register;
mod immediate;
mod verb;
mod keyword;
mod prepositions;
mod define;


pub use data_auto::*;
pub use data::{Data, DataSet};
pub use memory::Memory;
pub use label::Label;
pub use register::{RegType, Register};
pub use immediate::Immediate;
pub use keyword::Keyword;
pub use prepositions::{Preposition, PrepositionObject, PrepositionPhrases};
pub use define::{DefItem, Define};
pub use verb::Verb;
pub use code::{Sentence, Code};

pub type Result<T> = std::result::Result<T, ()>;

#[macro_export] macro_rules! emit_error_msg {
    ($msg:expr, $loc:expr) => {
        eprintln!("{}", format!("{}{}", $msg, $loc))
    };
}