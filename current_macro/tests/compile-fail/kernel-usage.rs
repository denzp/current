#![feature(proc_macro)]
#![cfg_attr(target_os = "cuda", no_std)]

extern crate current;
extern crate current_macro;

use current_macro::kernel;

#[kernel] //~ ERROR only functions can be kernels
mod mod_1 {}

#[kernel] //~ ERROR kernels should not have custom ABI
extern "C" fn kernel_1() {} //~ HELP Please remove `extern ...`

#[kernel] //~  ERROR kernels should not have generics
          //~^ HELP  Generics are coming soon.
fn kernel_2<T>(arg: T) {}

#[kernel] //~ ERROR kernels should not return anything
fn kernel_3() -> f64 {}
