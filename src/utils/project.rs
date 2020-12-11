use crate::utils::format;
use clap::arg_enum;
use dialoguer::{theme::ColorfulTheme, Select};
use include_dir_macro::include_dir;
use std::fs;
use std::{collections, path};

arg_enum! {
    #[derive(Debug)]
    pub enum ProjectType {
        JavaScript,
        ReScript,
        Rust,
    }
}

fn from_selection() -> ProjectType {
    let selections = &["ReScript", "Rust"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Multiple languages found, select one:")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selection {
        0 => ProjectType::ReScript,
        1 => ProjectType::Rust,
        _ => panic!("Unknown selection"),
    }
}

pub fn make() -> ProjectType {
    let has_bs_config = fs::metadata("bsconfig.json").is_ok();
    let has_cargo_toml = fs::metadata("Cargo.toml").is_ok();

    match (has_bs_config, has_cargo_toml) {
        (true, false) => ProjectType::ReScript,
        (false, true) => ProjectType::Rust,
        (true, true) => from_selection(),
        _ => ProjectType::JavaScript,
    }
}

pub struct Project {
    project_type: ProjectType,
}

impl Project {
    pub fn directory(&self) -> collections::HashMap<&'static path::Path, &'static [u8]> {
        match self.project_type {
            ProjectType::ReScript => include_dir!("src/templates/github_actions/rescript"),
            ProjectType::JavaScript => include_dir!("src/templates/github_actions/js"),
            ProjectType::Rust => include_dir!("src/templates/github_actions/rust"),
        }
    }

    pub fn gitignore(&self) -> &'static str {
        match self.project_type {
            ProjectType::ReScript => {
                include_str!("../templates/gitignore/.gitignore.res")
            }
            ProjectType::JavaScript => {
                include_str!("../templates/gitignore/.gitignore.js")
            }
            ProjectType::Rust => {
                include_str!("../templates/gitignore/.gitignore.rs")
            }
        }
    }

    pub fn release_config(&self) -> &'static str {
        match self.project_type {
            ProjectType::ReScript => {
                include_str!("../templates/github_actions/release_config/.releaserc.js")
            }
            ProjectType::JavaScript => {
                include_str!("../templates/github_actions/release_config/.releaserc.js")
            }
            ProjectType::Rust => {
                include_str!("../templates/github_actions/release_config/.releaserc.rs")
            }
        }
    }

    pub fn log(&self) {
        format::success(&format!("Found {} project", self.project_type))
    }

    pub fn new(project_type: Option<ProjectType>) -> Project {
        match project_type {
            Some(project_type) => Project { project_type },
            None => {
                let project_type = make();
                Project { project_type }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn js_project_string() {
        assert_eq!(format!("{}", &ProjectType::JavaScript), "JavaScript")
    }

    #[test]
    fn rust_project_string() {
        assert_eq!(format!("{}", &ProjectType::Rust), "Rust")
    }

    #[test]
    fn rescript_project_string() {
        assert_eq!(format!("{}", &ProjectType::ReScript), "ReScript")
    }
}
