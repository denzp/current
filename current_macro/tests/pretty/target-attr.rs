// pretty-compare-only
// pretty-mode:expanded
// pp-exact:target-attr.pp

#![feature(proc_macro, no_core)]
#![no_core]

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
