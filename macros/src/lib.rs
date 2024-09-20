extern crate proc_macro;
use proc_macro::TokenStream;

mod match_data;
use self::match_data::match_data_impl;

mod let_prep;
use self::let_prep::let_prep_impl;

mod gen_nasm;
use self::gen_nasm::gen_nasm_impl;

#[proc_macro]
pub fn match_data(args: TokenStream) -> TokenStream {
    match_data_impl(args)
}

#[proc_macro]
pub fn let_prep(args: TokenStream) -> TokenStream {
    let_prep_impl(args)
}

#[proc_macro]
pub fn gen_nasm(args: TokenStream) -> TokenStream {
    gen_nasm_impl(args)
}