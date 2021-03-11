use super::helpers;
use crate::config::{self, NodeInstaller};
use crate::utils::pkg_json;
use helpers::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref PKG: Regex = Regex::new(r"([\w@/-]+)\{([\w\-,]+)\}").unwrap();
}

struct Npm {}
struct Yarn {}

impl Npm {
    fn install(pkg: &str) {
        helpers::run_command("npm", &["install", "--save-exact", "--save-dev", pkg]);
    }

    fn uninstall(pkg: &str) {
        helpers::run_command("npm", &["uninstall", pkg]);
    }
}

impl Yarn {
    fn install(pkg: &str) {
        helpers::run_command("yarn", &["add", "--dev", pkg]);
    }

    fn uninstall(pkg: &str) {
        helpers::run_command("yarn", &["remove", pkg]);
    }
}

pub fn install_dev(pkg: &str) {
    let installer = match config::get().unwrap().node_installer {
        NodeInstaller::Npm => Npm::install,
        NodeInstaller::Yarn => Yarn::install,
    };

    packages(pkg).iter().for_each(|p| installer(p));
}

pub fn uninstall(pkg: &str) {
    let uninstaller = match config::get().unwrap().node_installer {
        NodeInstaller::Npm => Npm::uninstall,
        NodeInstaller::Yarn => Yarn::uninstall,
    };

    packages(pkg).iter().for_each(|p| uninstaller(p));
}

pub fn add_scripts(scripts: HashMap<&str, &str>) -> Result<()> {
    let mut pkg = pkg_json::Package::new()?;

    scripts.iter().for_each(|(name, cmd)| {
        pkg.scripts.insert(name.to_string(), cmd.to_string());
    });

    let json = serde_json::to_string_pretty(&pkg)?;

    fs::write("package.json", json)?;

    Ok(())
}

pub fn remove_scripts(scripts: Vec<&str>) -> Result<()> {
    let mut pkg = pkg_json::Package::new()?;

    scripts.iter().for_each(|name| {
        pkg.scripts.remove(&name.to_string());
    });

    let json = serde_json::to_string_pretty(&pkg)?;

    fs::write("package.json", json)?;

    Ok(())
}

fn split_packages(caps: regex::Captures) -> Option<Vec<String>> {
    let base = caps.get(1)?.as_str();

    Some(
        caps.get(2)?
            .as_str()
            .split(',')
            .map(|pkg| format!("{}{}", base, pkg))
            .collect(),
    )
}

fn packages(s: &str) -> Vec<String> {
    s.split_whitespace()
        .map(|s| match PKG.captures(s) {
            Some(caps) => split_packages(caps).unwrap(),
            None => vec![s.to_string()],
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_package_install() {
        assert_eq!(packages("prettier"), vec!["prettier"]);
    }

    #[test]
    fn parse_multiple_packages_install() {
        assert_eq!(packages("prettier meow"), vec!["prettier", "meow"]);
    }

    #[test]
    fn parse_multiple_packages_with_expansion() {
        assert_eq!(
            packages("prettier meow @testing-library/{react,cypress}"),
            vec![
                "prettier",
                "meow",
                "@testing-library/react",
                "@testing-library/cypress"
            ]
        );
    }

    #[test]
    fn parse_all_forms() {
        assert_eq!(
            packages(
                "prettier meow @testing-library/{react,jest-dom,react-hooks,cypress} eslint-plugin-{prettier,react}"
            ),
            vec![
                "prettier",
                "meow",
                "@testing-library/react",
                "@testing-library/jest-dom",
                "@testing-library/react-hooks",
                "@testing-library/cypress",
                "eslint-plugin-prettier",
                "eslint-plugin-react",
            ]
        );
    }
}
