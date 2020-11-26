use crate::utils::{format, helpers, template};
use dialoguer::{theme::ColorfulTheme, Select};
use helpers::Result;
use include_dir_macro::include_dir;
use serde_json::json;
use std::fs;

mod app_type {
    use super::*;

    pub enum T {
        JavaScript,
        ReScript,
        Rust,
    }

    fn from_selection() -> T {
        let selections = &["ReScript", "Rust"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Multiple languages found, select one:")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => T::ReScript,
            1 => T::Rust,
            _ => panic!("Unknown selection"),
        }
    }

    pub fn make() -> T {
        let has_bs_config = fs::metadata("bsconfig.json").is_ok();
        let has_cargo_toml = fs::metadata("Cargo.toml").is_ok();

        match (has_bs_config, has_cargo_toml) {
            (true, false) => T::ReScript,
            (false, true) => T::Rust,
            (true, true) => from_selection(),
            _ => T::JavaScript,
        }
    }
}

pub fn run(no_npm: bool) -> Result<()> {
    let (template_directory, releaserc) = match app_type::make() {
        app_type::T::ReScript => {
            format::success("Found ReScript project");

            (
                include_dir!("src/templates/github_actions/rescript"),
                include_str!("../templates/github_actions/release_config/.releaserc.js"),
            )
        }
        app_type::T::JavaScript => {
            format::success("Found JavaScript project");

            (
                include_dir!("src/templates/github_actions/js"),
                include_str!("../templates/github_actions/release_config/.releaserc.js"),
            )
        }
        app_type::T::Rust => {
            format::success("Found Rust project");

            (
                include_dir!("src/templates/github_actions/rust"),
                include_str!("../templates/github_actions/release_config/.releaserc.rs"),
            )
        }
    };

    let data = json!({ "name": env!("CARGO_PKG_NAME"), "noNpm": no_npm });

    template::render_dir(template_directory, ".github/workflows", &data)?;
    template::render_file(releaserc, ".releaserc", Some(&data))?;

    Ok(())
}
