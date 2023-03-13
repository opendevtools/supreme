use crate::utils::{helpers, node, pkg_json};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use helpers::Result;

pub fn run(name: Option<String>) -> Result<()> {
    let pkg = pkg_json::Package::new()?;

    let selections: Vec<_> = pkg
        .scripts
        .iter()
        .map(|(script, cmd)| format!("{} ({})", script, cmd))
        .collect();

    let scripts: Vec<_> = pkg
        .scripts
        .keys()
        .map(|script| script.to_string())
        .collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .default(0)
        .with_prompt("Select a script to run")
        .with_initial_text(match name {
            Some(name) => name,
            None => "".to_string(),
        })
        .items(&selections)
        .interact()
        .unwrap();

    node::run_script(&scripts[selection]);

    Ok(())
}
