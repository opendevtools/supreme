use crate::utils::{helpers, node};
use helpers::Result;

pub fn run(name: String, dev: bool) -> Result<()> {
    match dev {
        true => node::install_dev(&name),
        false => node::install(&name),
    };

    Ok(())
}
