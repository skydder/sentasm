use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Ident, Lit, Token};


struct ParsedData {
    var: Ident,
    prep: Lit
}

impl Parse for ParsedData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let var = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;
        let prep = input.parse::<Lit>()?;
        Ok(Self { var: var, prep: prep })
    }
}

pub(crate) fn let_prep_impl(args: TokenStream) -> TokenStream {
    let data = parse_macro_input!(args as ParsedData);
    let var = data.var;
    let prep = data.prep;
    quote!(
        let #var = sentence.preposition_phrases.get_object(Preposition(#prep)).map_or_else(|| None, |date| date.expect_object());
    ).into()
}