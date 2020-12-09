use crate::utils::format;
use dialoguer::{theme::ColorfulTheme, Select};
use include_dir_macro::include_dir;
use std::fs;
use std::{collections, path};

pub enum T {
    JavaScript,
    ReScript,
    Rust,
}

impl std::fmt::Display for T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            T::JavaScript => write!(f, "JavaScript"),
            T::ReScript => write!(f, "ReScript"),
            T::Rust => write!(f, "Rust"),
        }
    }
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

    let app_type = match (has_bs_config, has_cargo_toml) {
        (true, false) => T::ReScript,
        (false, true) => T::Rust,
        (true, true) => from_selection(),
        _ => T::JavaScript,
    };

    app_type
}

pub struct Project {
    app_type: T,
}

impl Project {
    pub fn directory(&self) -> collections::HashMap<&'static path::Path, &'static [u8]> {
        match self.app_type {
            T::ReScript => include_dir!("src/templates/github_actions/rescript"),
            T::JavaScript => include_dir!("src/templates/github_actions/js"),
            T::Rust => include_dir!("src/templates/github_actions/rust"),
        }
    }

    pub fn release_config(&self) -> &'static str {
        match self.app_type {
            T::ReScript => include_str!("../templates/github_actions/release_config/.releaserc.js"),
            T::JavaScript => {
                include_str!("../templates/github_actions/release_config/.releaserc.js")
            }
            T::Rust => include_str!("../templates/github_actions/release_config/.releaserc.rs"),
        }
    }

    pub fn log(&self) {
        format::success(&format!("Found {} project", self.app_type))
    }

    pub fn new() -> Project {
        let app_type = make();

        Project { app_type: app_type }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn js_project_string() {
        assert_eq!(format!("{}", &T::JavaScript), "JavaScript")
    }

    #[test]
    fn rust_project_string() {
        assert_eq!(format!("{}", &T::Rust), "Rust")
    }

    #[test]
    fn rescript_project_string() {
        assert_eq!(format!("{}", &T::ReScript), "ReScript")
    }
}
