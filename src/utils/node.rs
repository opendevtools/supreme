use super::helpers;
use crate::config::{self, NodeInstaller};
use crate::utils::pkg_json;
use colored::*;
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
    fn install(sync_lockfile: bool) {
        println!(
            "Installing dependencies using {manager}",
            manager = "npm".green()
        );
        match sync_lockfile {
            false => helpers::spawn_command("npm", &["install"]).unwrap(),
            true => helpers::spawn_command("npm", &["install", "--lockfile-only"]).unwrap(),
        };
    }

    fn install_pkg(pkg: &str) {
        helpers::spawn_command("npm", &["install", "--save-exact", pkg]).unwrap();
    }

    fn install_dev_pkg(pkg: &str) {
        helpers::spawn_command("npm", &["install", "--save-exact", "--save-dev", pkg]).unwrap();
    }

    fn uninstall(pkg: &str) {
        helpers::spawn_command("npm", &["uninstall", pkg]).unwrap();
    }

    fn update() {
        helpers::spawn_command("npx", &["npm-check-updates", "--interactive"]).unwrap();
    }

    fn run(script: &str) {
        helpers::spawn_command("npm", &["run", script]).expect("Could not start script");
    }
}

impl Yarn {
    fn install(_sync_lockfile: bool) {
        println!(
            "Installing dependencies using {manager}",
            manager = "yarn".green()
        );
        helpers::spawn_command("yarn", &["install"]).unwrap();
    }

    fn install_pkg(pkg: &str) {
        helpers::spawn_command("yarn", &["add", pkg]).unwrap();
    }

    fn install_dev_pkg(pkg: &str) {
        helpers::spawn_command("yarn", &["add", "--dev", pkg]).unwrap();
    }

    fn uninstall(pkg: &str) {
        helpers::spawn_command("yarn", &["remove", pkg]).unwrap();
    }

    fn update() {
        helpers::spawn_command("yarn", &["upgrade-interactive", "--latest"]).unwrap();
    }

    fn run(script: &str) {
        helpers::spawn_command("yarn", &[script]).expect("Could not start script");
    }
}

fn find_package_manager() -> config::NodeInstaller {
    match (fs::metadata("package-lock.json"), fs::metadata("yarn.lock")) {
        // Can't decide, use config
        (Err(_), Err(_)) | (Ok(_), Ok(_)) => config::get().unwrap().node_installer,
        (Ok(_), Err(_)) => NodeInstaller::Npm,
        (Err(_), Ok(_)) => NodeInstaller::Yarn,
    }
}

enum InstallationType {
    Install,
    InstallDev,
    Uninstall,
}

fn success_message(code: InstallationType, pkg: &str) {
    let text = match code {
        InstallationType::Install => "Installed",
        InstallationType::InstallDev => "Installed (dev)",
        InstallationType::Uninstall => "Uninstalled",
    };

    println!(
        "{check} {text} {pkg}",
        check = "✓".green(),
        text = text,
        pkg = pkg.blue()
    );
}

fn install_message(code: InstallationType, pkg: &str) {
    let text = match code {
        InstallationType::Install => "Installing",
        InstallationType::InstallDev => "Installing (dev)",
        InstallationType::Uninstall => "Uninstalling",
    };

    println!("⌛ {text} {pkg}", text = text, pkg = pkg.blue());
}

pub fn install_all(sync_lockfile: bool) {
    let installer = match find_package_manager() {
        NodeInstaller::Npm => Npm::install,
        NodeInstaller::Yarn => Yarn::install,
    };

    installer(sync_lockfile);
}

pub fn install(pkgs: &str) {
    let installer = match find_package_manager() {
        NodeInstaller::Npm => Npm::install_pkg,
        NodeInstaller::Yarn => Yarn::install_pkg,
    };

    packages(pkgs).iter().for_each(|p| {
        install_message(InstallationType::Install, p);
        installer(p);
        success_message(InstallationType::Install, p);
    });
}

pub fn install_dev(pkgs: &str) {
    let installer = match find_package_manager() {
        NodeInstaller::Npm => Npm::install_dev_pkg,
        NodeInstaller::Yarn => Yarn::install_dev_pkg,
    };

    packages(pkgs).iter().for_each(|p| {
        install_message(InstallationType::InstallDev, p);
        installer(p);
        success_message(InstallationType::InstallDev, p);
    });
}

pub fn uninstall(pkg: &str) {
    let uninstaller = match find_package_manager() {
        NodeInstaller::Npm => Npm::uninstall,
        NodeInstaller::Yarn => Yarn::uninstall,
    };

    packages(pkg).iter().for_each(|p| {
        install_message(InstallationType::Uninstall, p);
        uninstaller(p);
        success_message(InstallationType::Uninstall, p);
    });
}

pub fn update() -> Result<()> {
    let updater = match find_package_manager() {
        NodeInstaller::Npm => Npm::update,
        NodeInstaller::Yarn => Yarn::update,
    };

    updater();

    Ok(())
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

pub fn run_script(script: &str) {
    let script_runner = match find_package_manager() {
        NodeInstaller::Npm => Npm::run,
        NodeInstaller::Yarn => Yarn::run,
    };

    script_runner(script);
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
        .flat_map(|s| match PKG.captures(s) {
            Some(caps) => split_packages(caps).unwrap(),
            None => vec![s.to_string()],
        })
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
