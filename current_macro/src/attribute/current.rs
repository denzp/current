use proc_macro::{Diagnostic, Level, Span, TokenStream};
use proc_macro2;
use quote::*;
use syn::{parse, ExprTuple, Item};

pub fn expand_target_guard(args: TokenStream, input: TokenStream) -> Tokens {
    let input: Item = match parse(input.clone()) {
        Ok(item) => item,

        Err(_) => {
            return proc_macro2::TokenStream::from(input).into_tokens();
        }
    };

    match parse(args) {
        Ok(ExprTuple { elems, .. }) => {
            let attributes = match elems.into_tokens().to_string().as_ref() {
                "host" => quote!(#[cfg(not(target_os = "cuda"))]),
                "device" => quote!(#[cfg(target_os = "cuda")]),
                "shared" => quote!(),

                other @ _ => {
                    let error = format!("unknown crate type `{}`", other);

                    Diagnostic::spanned(Span::call_site(), Level::Error, error)
                        .help("You need to choose either `host`, `device` or `shared`.")
                        .emit();

                    quote!()
                }
            };

            quote! {
                #attributes
                #input
            }
        }

        Err(_) => {
            Diagnostic::spanned(Span::call_site(), Level::Error, "missing crate type")
                .help("You need to specify crate type like `#[current(host | device | shared)]`.")
                .emit();

            quote!(#input)
        }
    }
}
