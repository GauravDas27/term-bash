use std::env;
use std::process::Command;

fn main() {
    let mut cmmd = Command::new("C:/msys64/usr/bin/bash.exe");
    cmmd.arg("--login")
        .arg("-i")
        .env("CHERE_INVOKING", "1")
        .env("MSYS2_PATH_TYPE", "inherit")
        .env("PS1", "$ ");


    let mut argv = env::args();
    argv.next();
    loop {
        match argv.next() {
            None => break,
            Some(x) => cmmd.arg(x),
        };
    }

    cmmd.status().expect("could not start bash.exe");
}
