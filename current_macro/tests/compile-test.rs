extern crate compiletest_rs as compiletest;

use compiletest::common::Mode;
use std::path::PathBuf;

fn run_mode(mode: Mode) {
    let mut config = compiletest::Config::default();

    config.clean_rmeta();
    config.mode = mode;
    config.src_base = PathBuf::from(format!("tests/{}", mode));
    config.target_rustcflags = Some(String::from(
        "-L crate=../target/debug/ -L dependency=../target/debug/deps/",
    ));

    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode(Mode::CompileFail);
}
