use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use proc_macro::{self, TokenStream};
use proc_macro2::Span;
use quote::*;
use syn::{parse, FnDecl, Ident, ItemFn};

pub fn expand_kernel_fn(input: TokenStream) -> Tokens {
    let call_site = proc_macro::Span::call_site();

    let ItemFn {
        vis,
        ident,
        decl,
        block,
        ..
    } = parse(input).unwrap();

    let FnDecl {
        fn_token, inputs, ..
    } = *decl;

    // TODO: check ABI, Variadic, Output Type, Generics

    let kernel_id = {
        let mut hasher = DefaultHasher::new();

        call_site.source_file().path().hash(&mut hasher);
        call_site.start().line.hash(&mut hasher);
        call_site.start().column.hash(&mut hasher);
        call_site.end().line.hash(&mut hasher);
        call_site.end().column.hash(&mut hasher);

        format!("{}_{}", ident, hasher.finish())
    };

    let kernel_id_ident = Ident::new(&kernel_id, Span::call_site());

    quote! {
        #[no_mangle]
        #[cfg(target_os = "cuda")]
        pub unsafe extern "ptx-kernel" #fn_token #kernel_id_ident(#inputs) #block

        #[cfg(not(target_os = "cuda"))]
        #vis #fn_token #ident<E: ::current::CudaKernelExecutor>(executor: &E, #inputs) -> E::OutputTy {
            executor.execute_kernel(#kernel_id)
        }
    }
}
