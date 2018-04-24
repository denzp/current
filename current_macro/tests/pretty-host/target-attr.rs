// pretty-compare-only
// pretty-mode:expanded
// pp-exact:target-attr.pp

#![feature(proc_macro)]
#![cfg_attr(target_os = "cuda", feature(abi_ptx))]
#![cfg_attr(target_os = "cuda", no_std)]

extern crate current;
extern crate current_macro;

use current_macro::current;

#[current(host)]
mod mod_1 {}

#[current(device)]
mod mod_2 {}

#[current(shared)]
mod mod_3 {}

mod mod_4 {}
