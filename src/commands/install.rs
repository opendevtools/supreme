use crate::utils::{helpers, node};
use helpers::Result;

pub fn run(packages: Option<String>, dev: bool, sync_lockfile: bool) -> Result<()> {
    match (packages, dev, sync_lockfile) {
        (Some(packages), true, _) => node::install_dev(&packages),
        (Some(packages), false, _) => node::install(&packages),
        (None, _, sync_lockfile) => node::install_all(sync_lockfile),
    };

    Ok(())
}
