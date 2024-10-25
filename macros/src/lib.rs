extern crate proc_macro;
use proc_macro::TokenStream;

mod match_data;
use self::match_data::match_data_impl;

mod match_data2;
use self::match_data2::match_data2_impl;

mod get_prep_object;
use self::get_prep_object::get_prep_object_impl;

mod gen_nasm;
use self::gen_nasm::gen_nasm_impl;

mod make_operands;
use self::make_operands::make_operands_impl;

#[proc_macro]
pub fn match_data(args: TokenStream) -> TokenStream {
    match_data_impl(args)
}

#[proc_macro]
pub fn match_data2(args: TokenStream) -> TokenStream {
    match_data2_impl(args)
}


#[proc_macro]
pub fn gen_nasm(args: TokenStream) -> TokenStream {
    gen_nasm_impl(args)
}

#[proc_macro]
pub fn make_operands(args: TokenStream) -> TokenStream {
    make_operands_impl(args)
}

#[proc_macro]
pub fn get_prep_object(args: TokenStream) -> TokenStream {
    get_prep_object_impl(args)
}