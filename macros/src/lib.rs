extern crate proc_macro;
use proc_macro::TokenStream;
mod match_data;
use self::match_data::match_data_impl;

#[proc_macro]
pub fn match_data(args: TokenStream) -> TokenStream {
    match_data_impl(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn print() {
    //     eprintln!("{}", match_data!(Register(_, _, ..)))
    // }
}

