mod tokenizer;

pub use tokenizer::{Location, Tokenizer, Token, Stream, StreamInfo};

pub const PUNCTUATOR: &[&'static str] = & ["*(","-", "+", "*", "(", ")", "@[", "]", "[", ","];

pub fn _emit_error(location: Location, msg: String) {
    eprintln!("error: {}", msg);
    eprintln!(" --> {}", location);
}

#[macro_export]
macro_rules! emit_error {
    ($loc: expr, $($msgs: expr), *) => {
        eprintln!("error: {}", format!($($msgs),*));
        eprintln!(" --> {}", $loc);
        // _emit_error($loc, format!($($msgs),*))
    };
}