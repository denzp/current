// TODO: no_std for both targets?
#![cfg_attr(target_os = "cuda", no_std)]

pub trait CudaModuleLoader {
    fn get_module_ptx_assembly() -> &'static str;
}

pub trait CudaKernelExecutor {
    type OutputTy;

    fn execute_kernel(&self, name: &str) -> Self::OutputTy;
}
