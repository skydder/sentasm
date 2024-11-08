mod code;
mod data;
mod data_auto;
mod define;
mod immediate;
mod keyword;
mod label;
mod memory;
mod prepositions;
mod register;
mod verb;

pub use code::{Code, Sentence};
pub use data::{Data, DataSet};
pub use data_auto::*;
pub use define::{DefItem, Define};
pub use immediate::Immediate;
pub use keyword::Keyword;
pub use label::Label;
pub use memory::Memory;
pub use prepositions::{Preposition, PrepositionObject, PrepositionPhrases};
pub use register::{RegType, Register};
pub use verb::Verb;

pub type Result<T> = std::result::Result<T, ()>;

#[macro_export]
macro_rules! emit_error_msg {
    ($msg:expr, $loc:expr) => {
        eprintln!("{}", format!("{}{}", $msg, $loc))
    };
}
