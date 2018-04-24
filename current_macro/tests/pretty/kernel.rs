// pretty-compare-only
// pretty-mode:expanded
// pp-exact:kernel.pp

#![feature(proc_macro, no_core)]
#![no_core]

extern crate current;
extern crate current_macro;

use current_macro::kernel;

#[kernel]
fn kernel_1() {}

fn not_kernel_1() {}
