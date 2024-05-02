use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use include_dir_macro::include_dir;
use std::fmt::Display;
use std::fs;
use std::{collections, path};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ProjectType {
    JavaScript,
    ReScript,
    Rust,
}

impl Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let language = match self {
            ProjectType::JavaScript => "JavaScript",
            ProjectType::ReScript => "ReScript",
            ProjectType::Rust => "Rust",
        };

        write!(f, "{}", language)
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

impl ProjectType {
    fn new() -> Self {
        let has_bs_config = fs::metadata("bsconfig.json").is_ok();
        let has_cargo_toml = fs::metadata("Cargo.toml").is_ok();

        match (has_bs_config, has_cargo_toml) {
            (true, false) => ProjectType::ReScript,
            (false, true) => ProjectType::Rust,
            (true, true) => from_selection(),
            _ => ProjectType::JavaScript,
        }
    }
}

pub struct Project {
    pub project_type: ProjectType,
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
        println!("{} Found {} project", "✔".green(), self.project_type);
    }

    pub fn new(project_type: Option<ProjectType>) -> Project {
        match project_type {
            Some(project_type) => Project { project_type },
            None => Project {
                project_type: ProjectType::new(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn js_project_string() {
        assert_eq!(&ProjectType::JavaScript.to_string(), "JavaScript")
    }

    #[test]
    fn rust_project_string() {
        assert_eq!(&ProjectType::Rust.to_string(), "Rust")
    }

    #[test]
    fn rescript_project_string() {
        assert_eq!(&ProjectType::ReScript.to_string(), "ReScript")
    }
}
