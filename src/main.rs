use std::io::{self, Write};
use std::process::Command;

fn main() {
    let output = Command::new("cargo")
        .args(&["build"])
        .env("RUSTFLAGS", "-D warnings")
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    if !output.status.success() {
        Command::new("git")
            .args(&["reset", "--hard"])
            .status()
            .expect("failed to execute process");
    }
}
