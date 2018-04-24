use proc_macro::TokenStream;
use quote::*;
use syn::{parse, ExprTuple, Item};

pub fn expand_target_guard(args: TokenStream, input: TokenStream) -> Tokens {
    let input: Item = match parse(input.clone()) {
        Ok(item) => item,
        Err(_) => fallback!(input),
    };

    match parse(args) {
        Ok(ExprTuple { elems, .. }) => {
            let attributes = match elems.into_tokens().to_string().as_ref() {
                "host" => quote!(#[cfg(not(target_os = "cuda"))]),
                "device" => quote!(#[cfg(target_os = "cuda")]),
                "shared" => quote!(),

                other @ _ => {
                    call_site_error!(format!("unknown crate type `{}`", other))
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
            call_site_error!("missing crate type")
                .help("You need to specify crate type like `#[current(host | device | shared)]`.")
                .emit();

            quote!(#input)
        }
    }
}
