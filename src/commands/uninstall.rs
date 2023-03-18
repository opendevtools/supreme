use crate::utils::{helpers, node};
use helpers::Result;

pub fn run(name: Vec<String>) -> Result<()> {
    node::uninstall(&name);

    Ok(())
}
