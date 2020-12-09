use super::helpers;
use std::process;

pub fn install_dev(pkg: &str) -> process::Output {
    helpers::run_command("npm", &["install", "--save-exact", "--save-dev", pkg])
}
