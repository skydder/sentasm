// use proc_macro2::TokenStream as TokenStream2;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Ident};

// DataSet {data: Data::(), loc: _}

struct ParsedData(Ident, Expr);

impl Parse for ParsedData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = if let Ok(value) = input.parse::<Ident>() {
            value
        } else {
            panic!("value must be ident! t: {:?}", input);
        };
        let data = if let Ok(value) = input.parse::<Expr>() {
            value
        } else {
            panic!("value must be expr! t: {:?}", input);
        };
        Ok(Self(name, data))
    }
}

pub(crate) fn match_data_impl(args: TokenStream) -> TokenStream {
    let data = parse_macro_input!(args as ParsedData);
    let tup = match data.1 {
        Expr::Tuple(expr_tuple) => expr_tuple,
        // Expr::Struct(expr_struct) => expr_struct,
        _ => todo!(),
    };
    let name = data.0;
    quote!(
        Some(DataSet {data: Data::#name(#name #tup), loc: _})
    ).into()
}