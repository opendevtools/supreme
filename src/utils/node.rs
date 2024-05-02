use super::helpers;
use super::message::Message;
use crate::config::NodeInstaller;
use crate::utils::pkg_json;
use colored::*;
use helpers::Result;
use std::collections::HashMap;
use std::fs;

pub fn install_all(sync_lockfile: bool) {
    let package_manager = NodeInstaller::default();
    let arguments = match (package_manager, sync_lockfile) {
        (NodeInstaller::Npm, false)
        | (NodeInstaller::Yarn, _)
        | (NodeInstaller::Pnpm, _)
        | (NodeInstaller::Bun, _) => {
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

pub fn install(pkgs: &[String]) {
    let packages = pkgs.join(", ");
    let messager = Message::new(&packages);
    let package_manager = NodeInstaller::default();

    let mut arguments = match package_manager {
        NodeInstaller::Npm => vec!["install", "--save-exact"],
        NodeInstaller::Yarn => vec!["add", "--exact"],
        NodeInstaller::Pnpm => vec!["add", "--save-exact"],
        NodeInstaller::Bun => vec!["add", "--exact"],
    };

    pkgs.iter().for_each(|p| {
        arguments.push(p);
    });

    messager.install("Installing");

    helpers::spawn_command(&package_manager.to_string(), &arguments)
        .unwrap_or_else(|_| panic!("Failed to install {}", packages));

    messager.success("Installed");
}

pub fn install_dev(pkgs: &[String]) {
    let packages = pkgs.join(", ");
    let messager = Message::new(&packages);
    let package_manager = NodeInstaller::default();

    let mut arguments = match package_manager {
        NodeInstaller::Npm => vec!["install", "--save-exact", "--save-dev"],
        NodeInstaller::Yarn => vec!["add", "--exact", "--dev"],
        NodeInstaller::Pnpm => vec!["add", "--save-exact", "--save-dev"],
        NodeInstaller::Bun => vec!["add", "--exact", "--dev"],
    };

    pkgs.iter().for_each(|p| {
        arguments.push(p);
    });

    messager.install("Installing (dev)");

    helpers::spawn_command(&package_manager.to_string(), &arguments)
        .unwrap_or_else(|_| panic!("Failed to install {}", packages));

    messager.success("Installed (dev)");
}

pub fn install_global(pkgs: &[String]) {
    let packages = pkgs.join(", ");
    let messager = Message::new(&packages);
    let package_manager = NodeInstaller::default();

    let mut arguments = match package_manager {
        NodeInstaller::Npm => vec!["install", "--global"],
        NodeInstaller::Yarn => vec!["global", "add"],
        NodeInstaller::Pnpm => vec!["add", "--global"],
        NodeInstaller::Bun => vec!["add", "--global"],
    };

    pkgs.iter().for_each(|p| {
        arguments.push(p);
    });

    messager.install("Installing (globally)");

    helpers::spawn_command(&package_manager.to_string(), &arguments)
        .unwrap_or_else(|_| panic!("Failed to install {}", packages));

    messager.success("Installed (globally)");
}

pub fn uninstall(pkgs: &[String], global: bool) {
    let packages = pkgs.join(", ");
    let messager = Message::new(&packages);
    let package_manager = NodeInstaller::default();

    let mut arguments = match (package_manager, global) {
        (NodeInstaller::Npm, false) => vec!["uninstall"],
        (NodeInstaller::Npm, true) => vec!["uninstall", "--global"],
        (NodeInstaller::Yarn, false) => vec!["remove"],
        (NodeInstaller::Yarn, true) => vec!["global", "remove"],
        (NodeInstaller::Pnpm, false) => vec!["remove"],
        (NodeInstaller::Pnpm, true) => vec!["remove", "--global"],
        (NodeInstaller::Bun, false) => vec!["remove"],
        (NodeInstaller::Bun, true) => vec!["remove", "--global"],
    };

    pkgs.iter().for_each(|p| {
        arguments.push(p);
    });

    messager.install("Uninstalling");

    helpers::spawn_command(&package_manager.to_string(), &arguments)
        .unwrap_or_else(|_| panic!("Failed to uninstall {}", packages));

    messager.success("Uninstalled");
}

pub fn update() -> Result<()> {
    let package_manager = NodeInstaller::default();

    let arguments = match package_manager {
        NodeInstaller::Npm => vec!["npm-check-updates", "--interactive"],
        NodeInstaller::Yarn => {
            let output = helpers::run_command("yarn", &["-v"]);
            let output = String::from_utf8(output.stdout)?;
            let version = output.trim_end();
            let mut semver = version.split('.');
            let major = semver.next().unwrap();

            if major == "1" {
                vec!["upgrade-interactive", "--latest"]
            } else {
                vec!["upgrade-interactive"]
            }
        }
        NodeInstaller::Pnpm => vec!["update", "--interactive", "--latest"],
        NodeInstaller::Bun => panic!("Bun does not support updating dependencies"),
    };

    let package_runner = match package_manager {
        NodeInstaller::Npm => "npx",
        NodeInstaller::Yarn => "yarn",
        NodeInstaller::Pnpm => "pnpm",
        NodeInstaller::Bun => panic!("Bun does not support updating dependencies"),
    };

    helpers::spawn_command(package_runner, &arguments).unwrap();

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
        pkg.scripts.remove(*name);
    });

    let json = serde_json::to_string_pretty(&pkg)?;

    fs::write("package.json", json)?;

    Ok(())
}

pub fn run_script(script: &str) {
    let package_manager = NodeInstaller::default();
    let arguments = match package_manager {
        NodeInstaller::Npm | NodeInstaller::Pnpm | NodeInstaller::Bun => vec!["run", script],
        NodeInstaller::Yarn => vec![script],
    };

    helpers::spawn_command(&package_manager.to_string(), &arguments).unwrap();
}
