#![feature(proc_macro)]
#![cfg_attr(target_os = "cuda", no_std)]

extern crate current;
extern crate current_macro;

use current_macro::current;

#[current(host)]
fn func_1(2) {} //~ ERROR expected one of `...`, `..=`, `..`, or `:`, found `)`

#[current(host)]
mod mod_1() {} //~ ERROR expected one of `;` or `{`, found `(`
