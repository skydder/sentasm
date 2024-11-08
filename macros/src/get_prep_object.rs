use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Lit, Token};

struct ParsedData(Expr, Lit, Expr);

impl Parse for ParsedData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let sentence = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;
        let prep = input.parse::<Lit>()?;
        input.parse::<Token![,]>()?;
        let loc = input.parse::<Expr>()?;
        Ok(Self(sentence, prep, loc))
    }
}

pub(crate) fn get_prep_object_impl(args: TokenStream) -> TokenStream {
    let data = parse_macro_input!(args as ParsedData);
    let sentence = data.0;
    let prep = data.1;
    let loc = data.2;
    quote!(
        if let Some(data) = #sentence.preposition_phrases.get_object(Preposition(#prep)).map_or_else(|| None, |data| Some((data.0.expect_object(), data.1))) {
            (data.0, data.1)
        } else {
            (None, #loc)
        }
    ).into()
}
