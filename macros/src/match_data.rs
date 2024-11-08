use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, Pat, Token};

// DataSet {data: Data::(), loc: _}

struct ParsedData(Pat, Option<Ident>);

impl Parse for ParsedData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let pat = Pat::parse_multi(input)?;
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let var = input.parse::<Ident>()?;
            Ok(Self(pat, Some(var)))
        } else {
            Ok(Self(pat, None))
        }
    }
}

pub(crate) fn match_data_impl(args: TokenStream) -> TokenStream {
    let data = parse_macro_input!(args as ParsedData);
    let name = match &data.0 {
        Pat::TupleStruct(tup) => &tup.path,
        Pat::Struct(structs) => &structs.path,
        _ => todo!(),
    };
    let pat = &data.0;
    if data.1.is_some() {
        let loc = &data.1.unwrap();
        quote!(
            Some(DataSet {data: Data::#name(#pat), location: #loc, ..})
        )
        .into()
    } else {
        quote!(
            Some(DataSet {data: Data::#name(#pat), ..})
        )
        .into()
    }
}
