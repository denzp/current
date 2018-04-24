#![feature(prelude_import)]
#![no_std]
// pretty-compare-only
// pretty-mode:expanded
// pp-exact:target-attr.pp

#![feature(proc_macro)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;

extern crate current;
extern crate current_macro;

use current_macro::current;

#[cfg(not(target_os = "cuda"))]
mod mod_1 { }


mod mod_3 { }

mod mod_4 { }
