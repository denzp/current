extern crate compiletest_rs as compiletest;

use compiletest::common::Mode;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn run_mode(mode: Mode, base: Option<&str>, target: Option<&str>) {
    let mut config = compiletest::Config::default();

    config.clean_rmeta();
    config.mode = mode;

    config.target_rustcflags = Some(
        [
            "-L crate=../target/debug",
            "-L dependency=../target/debug/deps",
            "-L crate=../target/nvptx64-nvidia-cuda/debug",
            "-L dependency=../target/nvptx64-nvidia-cuda/debug/deps",
            &format!(
                "-L crate={}/../.xargo/lib/rustlib/nvptx64-nvidia-cuda/lib",
                env::var("CARGO_HOME").unwrap()
            ),
        ].join(" "),
    );

    config.src_base = match base {
        Some(base) => PathBuf::from(format!("tests/{}", base)),
        None => PathBuf::from(format!("tests/{}", mode)),
    };

    if let Some(target) = target {
        config.target = String::from(target);
    }

    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    prepare_nvptx_definition();
    build_nvptx_dependencies();

    run_mode(Mode::CompileFail, None, None);
    run_mode(Mode::CompileFail, None, Some("nvptx64-nvidia-cuda"));

    run_mode(Mode::Pretty, Some("pretty-host"), None);
    run_mode(
        Mode::Pretty,
        Some("pretty-device"),
        Some("nvptx64-nvidia-cuda"),
    );
}

fn prepare_nvptx_definition() {
    let mut shell = Command::new("sh");

    shell.args(&[
        "-c",
        "ptx-linker print nvptx64-nvidia-cuda > /tmp/nvptx64-nvidia-cuda.json",
    ]);

    assert!(shell.status().unwrap().success());
}

fn build_nvptx_dependencies() {
    let mut xargo = Command::new("xargo");

    xargo.current_dir("../current");
    xargo.env("RUST_TARGET_PATH", "/tmp");
    xargo.args(&["build", "--target", "nvptx64-nvidia-cuda"]);

    assert!(xargo.status().unwrap().success());
}
