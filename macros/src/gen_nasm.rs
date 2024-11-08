use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Lit, Token};

struct ParsedData(Lit, Vec<Expr>);

impl Parse for ParsedData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ins = input.parse::<Lit>()?;
        let mut operands: Vec<Expr> = Vec::new();
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
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
        Ok(Self(ins, operands))
    }
}

pub fn gen_nasm_impl(args: TokenStream) -> TokenStream {
    let data = parse_macro_input!(args as ParsedData);
    let ins = data.0;
    let operands = data.1;
    let mut operands_expression = "{}".to_string();

    if operands.len() != 0 {
        for _ in 0..operands.len() - 1 {
            operands_expression.push_str(" {},");
        }
        operands_expression.push_str(" {}");
        let ins_set = quote! {#operands_expression};
        // eprintln!("{}", operands_expression);
        quote! {
            format!(#ins_set, #ins, #(#operands),*)
        }
        .into()
    } else {
        quote! {format!(#ins)}.into()
    }
}
