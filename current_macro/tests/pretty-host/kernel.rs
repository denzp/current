// pretty-compare-only
// pretty-mode:expanded
// pp-exact:kernel.pp

#![feature(proc_macro)]
#![cfg_attr(target_os = "cuda", feature(abi_ptx))]
#![cfg_attr(target_os = "cuda", no_std)]

extern crate current;
extern crate current_macro;

use current_macro::kernel;

#[kernel]
fn kernel_1() {}

fn not_kernel_1() {}
