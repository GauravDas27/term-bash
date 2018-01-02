use std::process::Command;

fn main() {
    Command::new("C:/msys64/usr/bin/bash.exe")
        .arg("--login")
        .arg("-i")
        .env("CHERE_INVOKING", "1")
        .env("MSYS2_PATH_TYPE", "inherit")
        .env("PS1", "$ ")
        .status()
        .expect("could not start bash.exe");
}
