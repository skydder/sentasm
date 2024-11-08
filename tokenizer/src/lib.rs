mod tokenizer;

pub use tokenizer::{Location, Stream, StreamInfo, Token, Tokenizer};

pub const PUNCTUATOR: &[&'static str] =
    &["*(", "-", "+", "*", "(", ")", "@[", "]", "[", ",", "@", "#"];

pub fn _emit_error(location: Location, msg: String) {
    eprintln!("error: {}", msg);
    eprintln!(" --> {}", location);
}

#[macro_export]
macro_rules! emit_error {
    ($loc: expr, $($msgs: expr), *) => {
        $crate::_emit_error($loc, format!($($msgs),*));
        ::std::process::exit(1);
    };
}
