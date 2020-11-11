use std::process;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn run_command(cmd: &str, arg: &[&str]) -> process::Output {
    process::Command::new(cmd)
        .args(arg)
        .output()
        .expect("Failed to execute command")
}

pub fn install_dev(pkg: &str) -> process::Output {
    run_command("npm", &["install", "--save-exact", "--save-dev", pkg])
}
