use crate::utils::{helpers, node};
use helpers::Result;

pub fn run(packages: Vec<String>, dev: bool, sync_lockfile: bool, global: bool) -> Result<()> {
    match (packages, dev, sync_lockfile, global) {
        (packages, _, _, true) => node::install_global(&packages),
        (packages, _, sync_lockfile, false) if packages.is_empty() => {
            node::install_all(sync_lockfile)
        }
        (packages, true, _, false) => node::install_dev(&packages),
        (packages, false, _, false) => node::install(&packages),
    };

    Ok(())
}
