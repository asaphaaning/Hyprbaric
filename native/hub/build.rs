use std::{
    env,
    path::PathBuf,
    process::{Command, Stdio},
};

fn main() {
    println!("cargo:rerun-if-changed=../hyprbaric/src/signals");

    let project_root = project_root();
    let status = Command::new("rinf")
        .arg("gen")
        .current_dir(&project_root)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap_or_else(|error| panic!("failed to execute `rinf gen`: {error}"));

    if !status.success() {
        panic!("`rinf gen` exited with status {status}");
    }
}

fn project_root() -> PathBuf {
    let mut dir = env::current_dir()
        .unwrap_or_else(|error| panic!("failed to resolve current directory: {error}"));
    // native/hub -> native
    dir.pop();
    // native -> project root
    dir.pop();
    dir
}
