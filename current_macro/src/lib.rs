#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

mod derive;
use derive::derive_module_loader;

mod attribute;
use attribute::{expand_kernel_fn, expand_target_guard};

#[proc_macro_derive(CudaModuleLoader)]
pub fn module_loader(input: TokenStream) -> TokenStream {
    derive_module_loader(input).into()
}

#[proc_macro_attribute]
pub fn kernel(_args: TokenStream, input: TokenStream) -> TokenStream {
    expand_kernel_fn(input).into()
}

#[proc_macro_attribute]
pub fn current(args: TokenStream, input: TokenStream) -> TokenStream {
    expand_target_guard(args, input).into()
}
