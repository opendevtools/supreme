use crate::utils::{helpers, node};
use helpers::Result;

pub fn run(name: String) -> Result<()> {
    node::uninstall(&name);

    Ok(())
}
