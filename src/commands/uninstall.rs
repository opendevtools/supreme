use crate::utils::{helpers, node};
use helpers::Result;

pub fn run(name: Vec<String>, global: bool) -> Result<()> {
    node::uninstall(&name, global);

    Ok(())
}
