#![feature(proc_macro, plugin)]
#![plugin(current_plugin)]
// The plugin is needed because we can't easily apply proc_macro_attr to whole crate
// https://github.com/rust-lang/rust/issues/41430

// ⚡current⚡ required crate attributes
#![current_crate]
#![cfg_attr(target_os = "cuda", feature(abi_ptx))]
#![cfg_attr(target_os = "cuda", no_std)]

extern crate current;
extern crate current_macro;

mod mod_1 {
    fn host_only() {}
}

mod mod_2 {
    use current_macro::CudaModuleLoader;

    #[derive(CudaModuleLoader)]
    struct Test2 {}
}

mod mod_3 {
    use current_macro::current;

    #[current(device)]
    fn device_only() {}

    #[current(host)]
    fn host_only() {}
}

mod mod_4 {
    use current_macro::kernel;

    #[kernel]
    fn test1(x: &[f32], y: &mut [f32]) {
        y[0] = x[0];
    }
}

mod mod_5 {
    use current_macro::current;

    #[current(shared)]
    fn shared() {}
}
