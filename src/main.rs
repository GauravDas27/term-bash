use std::process::Command;

fn main() {
    let mut cmd = Command::new("C:/msys64/usr/bin/bash.exe");
    cmd.arg("--login")
        .arg("-i")
        .env("CHERE_INVOKING", "1")
        .env("MSYS2_PATH_TYPE", "inherit")
        .env("PS1", "$ ");
    cmd.status().expect("could not start bash.exe");
}
