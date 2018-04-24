use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use proc_macro::{Span, TokenStream};
use proc_macro2;
use quote::*;
use syn::{parse, FnDecl, Ident, Item, ItemFn, ReturnType};

pub fn expand_kernel_fn(input: TokenStream) -> Tokens {
    let call_site = Span::call_site();

    if parse::<Item>(input.clone()).is_err() {
        fallback!(input);
    }

    match parse::<ItemFn>(input.clone()) {
        Ok(item) => {
            if let Some(abi) = item.abi {
                call_site_error!("kernels should not have custom ABI")
                    .span_help(abi.extern_token.0.unstable(), "Please remove `extern ...`.")
                    .emit();

                fallback!(input);
            }

            if item.decl.generics.params.len() > 0 {
                call_site_error!("kernels should not have generics")
                    .help("Generics are coming soon.")
                    .emit();

                fallback!(input);
            }

            match item.decl.output {
                ReturnType::Type(_, _) => {
                    call_site_error!("kernels should not return anything").emit();
                    fallback!(input);
                }

                _ => {}
            }

            let kernel_id = {
                let mut hasher = DefaultHasher::new();

                call_site.source_file().path().hash(&mut hasher);
                call_site.start().line.hash(&mut hasher);
                call_site.start().column.hash(&mut hasher);
                call_site.end().line.hash(&mut hasher);
                call_site.end().column.hash(&mut hasher);

                format!("{}_{}", item.ident, hasher.finish())
            };

            // TODO: should this point to original location?
            let kernel_id_ident = Ident::new(&kernel_id, proc_macro2::Span::call_site());

            {
                let ItemFn {
                    vis,
                    ident,
                    decl,
                    block,
                    ..
                } = item;

                let FnDecl {
                    fn_token, inputs, ..
                } = *decl;

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
        }

        Err(_) => {
            call_site_error!("only functions can be kernels").emit();
            fallback!(input);
        }
    }
}
