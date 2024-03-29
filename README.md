# ⚡ Current ⚡
**CUDA high-level Rust framework**

## Warning
The crate is a **far-far away** from being usable.

## Hacking
You can inspect host and device code with a help of `xargo`, `cargo-expand` and `ptx-linker` (all can be installed from crates.io):

``` bash
ptx-linker print nvptx64-nvidia-cuda > /tmp/nvptx64-nvidia-cuda.json
export RUST_TARGET_PATH=/tmp

cd example

cargo expand                              # host-side version
xargo expand --target nvptx64-nvidia-cuda # device-side version
```

## Running tests
You need to prepare target json and env variable with the json location for `rustc`:

``` bash
ptx-linker print nvptx64-nvidia-cuda > /tmp/nvptx64-nvidia-cuda.json
export RUST_TARGET_PATH=/tmp

cargo test
```