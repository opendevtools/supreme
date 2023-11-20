use crate::utils::{helpers, node};
use helpers::Result;

pub fn run(packages: Vec<String>, dev: bool, sync_lockfile: bool, global: bool) -> Result<()> {
    let packages = packages
        .iter()
        // Filter out any flags
        // This can happen when passing args using `--`
        // For example: `supreme install is-even -- -W`
        .filter(|p| !p.starts_with('-'))
        .map(|p| p.to_string())
        .collect::<Vec<String>>();

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
