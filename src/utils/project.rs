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

fn log(app_type: &T) {
    format::success(&format!("Found {} project", app_to_string(app_type)))
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

pub fn app_to_string(app_type: &T) -> &'static str {
    match app_type {
        T::JavaScript => "JavaScript",
        T::ReScript => "ReScript",
        T::Rust => "Rust",
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

    log(&app_type);

    app_type
}

struct Location<'a> {
    app_type: &'a T,
}

impl<'a> Location<'a> {
    fn directory(&self) -> collections::HashMap<&'static path::Path, &'static [u8]> {
        match self.app_type {
            T::ReScript => include_dir!("src/templates/github_actions/rescript"),
            T::JavaScript => include_dir!("src/templates/github_actions/js"),
            T::Rust => include_dir!("src/templates/github_actions/rust"),
        }
    }

    fn release_config(&self) -> &'static str {
        match self.app_type {
            T::ReScript => include_str!("../templates/github_actions/release_config/.releaserc.js"),
            T::JavaScript => {
                include_str!("../templates/github_actions/release_config/.releaserc.js")
            }
            T::Rust => include_str!("../templates/github_actions/release_config/.releaserc.rs"),
        }
    }

    fn new(app_type: &T) -> Location {
        Location {
            app_type: &app_type,
        }
    }
}

pub struct Project<'a> {
    pub directory: collections::HashMap<&'a path::Path, &'a [u8]>,
    pub release_config: String,
}

impl<'a> Project<'a> {
    pub fn new() -> Project<'a> {
        let app_type = make();
        let locations = Location::new(&app_type);

        Project {
            directory: locations.directory(),
            release_config: locations.release_config().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn js_project_string() {
        assert_eq!(app_to_string(&T::JavaScript), "JavaScript")
    }

    #[test]
    fn rust_project_string() {
        assert_eq!(app_to_string(&T::Rust), "Rust")
    }

    #[test]
    fn rescript_project_string() {
        assert_eq!(app_to_string(&T::ReScript), "ReScript")
    }
}
