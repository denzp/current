#![feature(prelude_import)]
#![no_std]
// pretty-compare-only
// pretty-mode:expanded
// pp-exact:target-attr.pp

#![feature(proc_macro)]
#![feature(abi_ptx)]
#![no_std]
#[prelude_import]
use core::prelude::v1::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;

extern crate current;
extern crate current_macro;

use current_macro::current;


#[cfg(target_os = "cuda")]
mod mod_2 { }

mod mod_3 { }

mod mod_4 { }
