use crate::utils::{helpers, node, pkg_json};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use helpers::Result;

pub fn run() -> Result<()> {
    let pkg = pkg_json::Package::new()?;

    let selections: Vec<_> = pkg
        .scripts
        .iter()
        .map(|(script, cmd)| format!("{} ({})", script.bold(), cmd.magenta()))
        .collect();

    let scripts: Vec<_> = pkg
        .scripts
        .iter()
        .map(|(script, _)| format!("{}", script))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a script to run")
        .items(&selections)
        .interact()
        .unwrap();

    node::run_script(&scripts[selection]);

    Ok(())
}
