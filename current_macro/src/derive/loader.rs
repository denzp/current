use proc_macro::TokenStream;
use quote::*;
use syn::{parse, DeriveInput};

pub fn derive_module_loader(input: TokenStream) -> Tokens {
    let DeriveInput { ident, .. } = parse(input).unwrap();

    let output = quote! {
        #[cfg(not(target_os = "cuda"))]
        impl ::current::CudaModuleLoader for #ident {
            fn get_module_ptx_assembly() -> &'static str {
                "TODO: real ptx assembly"
            }
        }
    };

    output
}
