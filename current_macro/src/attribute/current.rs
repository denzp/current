use proc_macro::TokenStream;
use quote::*;
use syn::{parse, Item, ExprTuple};

pub fn expand_target_guard(args: TokenStream, input: TokenStream) -> Tokens {
    let input: Item = parse(input).unwrap();
    let ExprTuple { elems, .. } = parse(args).unwrap();

    // TODO: test wrong or empty args

    let attributes = match elems.into_tokens().to_string().as_ref() {
        "host" => quote!(#[cfg(not(target_os = "cuda"))]),
        "device" => quote!(#[cfg(target_os = "cuda")]),
        "shared" | _ => quote!(),
    };

    quote! {
        #attributes
        #input
    }
}
