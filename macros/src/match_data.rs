use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Pat};

// DataSet {data: Data::(), loc: _}

struct ParsedData(Pat);

impl Parse for ParsedData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let pat = Pat::parse_multi(input)?;
        Ok(Self(pat))
    }
}

pub(crate) fn match_data_impl(args: TokenStream) -> TokenStream {
    let data = parse_macro_input!(args as ParsedData);
    let name = match &data.0 {
        Pat::TupleStruct(tup) => {
            &tup.path
        },
        Pat::Struct(structs) => {
            &structs.path
        }
        _ => todo!(),
    };
    let pat = &data.0;
    quote!(
        Some(DataSet {data: Data::#name(#pat), loc: _})
    ).into()
}