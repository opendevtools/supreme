use crate::utils::{helpers, node};
use helpers::Result;

pub fn run(packages: Option<String>, dev: bool) -> Result<()> {
    match (packages, dev) {
        (Some(packages), true) => node::install_dev(&packages),
        (Some(packages), false) => node::install(&packages),
        (None, _) => node::install_all(),
    };

    Ok(())
}
