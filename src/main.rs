use std::env;
use std::process::Command;

fn main() {
    let mut cmmd = Command::new("C:/msys64/usr/bin/bash.exe");
    cmmd.arg("--login")
        .arg("-i")
        .env("CHERE_INVOKING", "1")
        .env("MSYS2_PATH_TYPE", "inherit")
        .env("PS1", "$ ");


    let argv = env::args().skip(1);
    for arg in argv {
        cmmd.arg(arg);
    }

    cmmd.status().unwrap_or_else(|err| {
        eprintln!("could not start bash.exe\n{}", err);
        std::process::exit(1)
    });
}
