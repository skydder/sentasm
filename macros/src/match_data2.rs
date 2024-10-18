use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, Pat, Token};

// DataSet {data: Data::(), loc: _}

struct ParsedData(Ident, Pat, Option<Ident>);

impl Parse for ParsedData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let data = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;
        let pat = Pat::parse_multi(input)?;
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let var = input.parse::<Ident>()?;
            Ok(Self(data, pat, Some(var)))
        } else {
            Ok(Self(data, pat, None))
        }
    }
}

pub(crate) fn match_data2_impl(args: TokenStream) -> TokenStream {
    let data = parse_macro_input!(args as ParsedData);
    let name = &data.0;
    let pat = &data.1;
    if data.2.is_some() {
        let loc = &data.2.unwrap();
        quote!(
            DataSet {data: Data::#name(#pat), location: #loc, ..}   
        ).into()
    } else {
        quote!(
            DataSet {data: Data::#name(#pat), ..}
        ).into()
    }
    
}