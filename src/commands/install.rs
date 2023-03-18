use crate::utils::{helpers, node};
use helpers::Result;

pub fn run(packages: Vec<String>, dev: bool, sync_lockfile: bool) -> Result<()> {
    match (packages, dev, sync_lockfile) {
        (packages, _, sync_lockfile) if packages.is_empty() => node::install_all(sync_lockfile),
        (packages, true, _) => node::install_dev(&packages),
        (packages, false, _) => node::install(&packages),
    };

    Ok(())
}
