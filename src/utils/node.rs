use super::helpers;
use super::message::Message;
use super::packages::packages;
use crate::config::NodeInstaller;
use crate::utils::pkg_json;
use colored::*;
use helpers::Result;
use std::collections::HashMap;
use std::fs;

pub fn install_all(sync_lockfile: bool) {
    let package_manager = NodeInstaller::default();
    let arguments = match (package_manager, sync_lockfile) {
        (NodeInstaller::Npm, false) | (NodeInstaller::Yarn, _) | (NodeInstaller::Pnpm, _) => {
            vec!["install"]
        }
        (NodeInstaller::Npm, true) => vec!["install", "--lockfile-only"],
    };

    println!(
        "Installing dependencies using {}",
        package_manager.to_string().green()
    );

    helpers::spawn_command(&package_manager.to_string(), &arguments).expect("Failed to install");
}

pub fn install(pkgs: &str) {
    let package_manager = NodeInstaller::default();
    let mut arguments = match package_manager {
        NodeInstaller::Npm => vec!["install", "--save-exact"],
        NodeInstaller::Yarn => vec!["add"],
        NodeInstaller::Pnpm => vec!["add", "--save-exact"],
    };

    packages(pkgs).iter().for_each(|p| {
        let messager = Message::new(p);
        arguments.push(p);

        messager.install("Installing", &package_manager.to_string());
        helpers::spawn_command(&package_manager.to_string(), &arguments)
            .expect(&format!("Failed to install {}", p));
        messager.success("Installed");
    });
}

pub fn install_dev(pkgs: &str) {
    let package_manager = NodeInstaller::default();
    let mut arguments = match package_manager {
        NodeInstaller::Npm => vec!["install", "--save-exact", "--save-dev"],
        NodeInstaller::Yarn => vec!["add", "--dev"],
        NodeInstaller::Pnpm => vec!["add", "--save-exact", "--save-dev"],
    };

    packages(pkgs).iter().for_each(|p| {
        let messager = Message::new(p);
        arguments.push(p);

        messager.install("Installing (dev)", &package_manager.to_string());
        helpers::spawn_command(&package_manager.to_string(), &arguments)
            .expect(&format!("Failed to install {}", p));
        messager.success("Installed (dev)");
    });
}

pub fn uninstall(pkg: &str) {
    let package_manager = NodeInstaller::default();
    let mut arguments = match package_manager {
        NodeInstaller::Npm => vec!["uninstall"],
        NodeInstaller::Yarn | NodeInstaller::Pnpm => vec!["remove"],
    };

    packages(pkg).iter().for_each(|p| {
        let messager = Message::new(p);
        arguments.push(p);

        messager.install("Uninstalling", &package_manager.to_string());
        helpers::spawn_command(&package_manager.to_string(), &arguments)
            .expect(&format!("Failed to uninstall {}", p));
        messager.success("Uninstalled");
    });
}

pub fn update() -> Result<()> {
    let package_manager = NodeInstaller::default();
    let arguments = match package_manager {
        NodeInstaller::Npm => vec!["npm-check-updates", "--interactive"],
        NodeInstaller::Yarn => vec!["upgrade-interactive", "--latest"],
        NodeInstaller::Pnpm => vec!["update", "--interactive", "--latest"],
    };

    helpers::spawn_command(&package_manager.to_string(), &arguments).unwrap();

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
    let package_manager = NodeInstaller::default();
    let arguments = match package_manager {
        NodeInstaller::Npm | NodeInstaller::Pnpm => vec!["run", script],
        NodeInstaller::Yarn => vec![script],
    };

    helpers::spawn_command(&package_manager.to_string(), &arguments).unwrap();
}
