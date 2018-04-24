#![feature(prelude_import)]
#![no_std]
// pretty-compare-only
// pretty-mode:expanded
// pp-exact:kernel.pp

#![feature(proc_macro)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;

extern crate current;
extern crate current_macro;

use current_macro::kernel;

#[cfg(not(target_os = "cuda"))]
fn kernel_1<E: ::current::CudaKernelExecutor>(executor: &E) -> E::OutputTy {
    executor.execute_kernel("kernel_1_15083529970317753591")
}

fn not_kernel_1() { }
