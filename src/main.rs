use std::env;
use std::io::{self, Result, Write};
use std::process::Command;

fn main() -> Result<()> {
    let output = Command::new("cargo")
        .args(&["build"])
        .env("RUSTFLAGS", "-D warnings")
        .output()?;
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    if !output.status.success() {
        let status = Command::new("git").args(&["reset", "--hard"]).status()?;
        if !status.success() {
            println!("I don't play like that! Install 'git', wimp!");
        }
        if env::args().any(|x| x == "--god") {
            kill("emacs");
            kill("vim");
        }
    }
    Ok(())
}

fn kill(cmd: &str) {
    Command::new("killall").args(&["-9", cmd]).status().ok();
}
