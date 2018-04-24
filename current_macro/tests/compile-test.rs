extern crate compiletest_rs as compiletest;

use compiletest::common::Mode;
use std::path::PathBuf;

fn run_mode(mode: Mode, target: Option<&str>) {
    let mut config = compiletest::Config::default();

    config.clean_rmeta();
    config.mode = mode;
    config.src_base = PathBuf::from(format!("tests/{}", mode));
    config.target_rustcflags = Some(String::from(
        "-L crate=../target/debug/ -L dependency=../target/debug/deps/",
    ));

    if let Some(target) = target {
        config.target = String::from(target);
    }

    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode(Mode::CompileFail, None);
    // run_mode(Mode::CompileFail, Some("nvptx64-nvidia-cuda"));

    run_mode(Mode::Pretty, None);
    // run_mode(Mode::Pretty, Some("nvptx64-nvidia-cuda"));
}
