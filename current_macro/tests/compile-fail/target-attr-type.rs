#![feature(proc_macro)]
#![cfg_attr(target_os = "cuda", no_std)]

extern crate current;
extern crate current_macro;

use current_macro::current;

#[current] //~  ERROR missing crate type
           //~^ HELP  You need to specify crate type like `#[current(host | device | shared)]`.
fn func_1() {}

#[current(non_existing_target)] //~  ERROR unknown crate type `non_existing_target`
                                //~^ HELP  You need to choose either `host`, `device` or `shared`.
mod mod_1 {}
