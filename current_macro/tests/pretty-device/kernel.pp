#![feature(prelude_import)]
#![no_std]
// pretty-compare-only
// pretty-mode:expanded
// pp-exact:kernel.pp

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

use current_macro::kernel;

#[no_mangle]
#[cfg(target_os = "cuda")]
pub unsafe extern "ptx-kernel" fn kernel_1_15083529970317753591() { }

fn not_kernel_1() { }
