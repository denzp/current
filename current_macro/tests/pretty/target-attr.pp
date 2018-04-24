// pretty-compare-only
// pretty-mode:expanded
// pp-exact:target-attr.pp

#![feature(proc_macro, no_core)]
#![no_core]

extern crate current;
extern crate current_macro;

use current_macro::current;

#[cfg(not(target_os = "cuda"))]
mod mod_1 { }


mod mod_3 { }

mod mod_4 { }
