use crate::utils::{helpers, node};
use helpers::Result;

pub fn run(packages: Vec<String>, global: bool) -> Result<()> {
    let packages = packages
        .iter()
        // Filter out any flags
        // This can happen when passing args using `--`
        // For example: `supreme install is-even -- -W`
        .filter(|p| !p.starts_with('-'))
        .map(|p| p.to_string())
        .collect::<Vec<String>>();

    node::uninstall(&packages, global);

    Ok(())
}
