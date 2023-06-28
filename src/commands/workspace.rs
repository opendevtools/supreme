use crate::{
    config::NodeInstaller,
    utils::{helpers, message},
};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::fs;

use helpers::{spawn_command, Result};
use message::Message;

fn get_targeted_package() -> Result<String> {
    // Find directory names inside the packages folder
    let current_packages = fs::read_dir("packages")?;
    let mut package_names = vec![];

    for entry in current_packages {
        let entry = entry?;
        let path = entry.path();
        let path = path.to_str().unwrap();
        let path = path.replace("packages/", "");
        package_names.push(path);
    }

    // Select a package
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .default(0)
        .with_prompt("Select a package")
        .items(&package_names)
        .interact()
        .unwrap();

    Ok(package_names[selection].to_string())
}

pub fn add(pkgs: Vec<String>, dev: bool) -> Result<()> {
    if fs::metadata("yarn.lock").is_err() {
        return Err("This doesn't seem like a Yarn project".into());
    }

    let packages = pkgs.join(", ");
    let messager = Message::new(&packages);
    let package_name = get_targeted_package()?;

    let mut arguments = vec!["workspace", &package_name, "add", "--exact"];

    if dev {
        arguments.push("--dev");
    }

    pkgs.iter().for_each(|p| {
        arguments.push(p);
    });

    messager.install("Installing");

    spawn_command(&NodeInstaller::Yarn.to_string(), &arguments)
        .unwrap_or_else(|_| panic!("Failed to install {} to workspace", packages));

    messager.success("Installed");

    Ok(())
}

pub fn remove(pkgs: Vec<String>) -> Result<()> {
    let packages = pkgs.join(", ");
    let messager = Message::new(&packages);
    let package_name = get_targeted_package()?;

    let mut arguments = vec!["workspace", &package_name, "remove"];

    pkgs.iter().for_each(|p| {
        arguments.push(p);
    });

    messager.install("Uninstalling");

    helpers::spawn_command(&NodeInstaller::Yarn.to_string(), &arguments)
        .unwrap_or_else(|_| panic!("Failed to uninstall {}", packages));

    messager.success("Uninstalled");

    Ok(())
}
