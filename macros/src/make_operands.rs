use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Token};

struct ParsedData(Vec<Expr>);

impl Parse for ParsedData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut operands: Vec<Expr> = Vec::new();
        while !input.is_empty() {
            operands.push(if let Ok(expr) = input.parse::<Expr>() {
                expr
            } else {
                panic!("expr should have come");
            });
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Self(operands))
    }
}

pub fn make_operands_impl(args: TokenStream) -> TokenStream {
    let data = parse_macro_input!(args as ParsedData);
    let operands = data.0;

    if operands.len() > 4 {
        panic!("invalid operands");
    }
    let mut op = Vec::new();
    for i in 0..4 {
        if let Some(operand) = operands.get(i) {
            op.push(quote! {#operand});
        } else {
            op.push(quote! {None});
        }
    }

    quote! {Operands(#(#op),*)}.into()
}
