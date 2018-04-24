// pretty-compare-only
// pretty-mode:expanded
// pp-exact:kernel.pp

#![feature(proc_macro, no_core)]
#![no_core]

extern crate current;
extern crate current_macro;

use current_macro::kernel;

#[cfg(not(target_os = "cuda"))]
fn kernel_1<E: ::current::CudaKernelExecutor>(executor: &E) -> E::OutputTy {
    executor.execute_kernel("kernel_1_9207892068069266964")
}

fn not_kernel_1() { }
