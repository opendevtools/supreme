extern crate confy;

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(clap::ValueEnum, Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum NodeInstaller {
    Npm,
    Yarn,
    Pnpm,
    Bun,
}

impl ToString for NodeInstaller {
    fn to_string(&self) -> String {
        match self {
            NodeInstaller::Npm => "npm".to_string(),
            NodeInstaller::Yarn => "yarn".to_string(),
            NodeInstaller::Pnpm => "pnpm".to_string(),
            NodeInstaller::Bun => "bun".to_string(),
        }
    }
}

impl Default for NodeInstaller {
    fn default() -> Self {
        match (
            fs::metadata("package-lock.json"),
            fs::metadata("yarn.lock"),
            fs::metadata("pnpm-lock.yaml"),
            fs::metadata("bun.lockb"),
        ) {
            (Ok(_), Err(_), Err(_), Err(_)) => NodeInstaller::Npm,
            (Err(_), Ok(_), Err(_), Err(_)) => NodeInstaller::Yarn,
            (Err(_), Err(_), Ok(_), Err(_)) => NodeInstaller::Pnpm,
            (Err(_), Err(_), Err(_), Ok(_)) => NodeInstaller::Bun,
            // Can't decide, use config
            _ => get().unwrap().node_installer,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupremeConfig {
    pub node_installer: NodeInstaller,
}

impl Default for SupremeConfig {
    fn default() -> Self {
        Self {
            node_installer: NodeInstaller::Npm,
        }
    }
}

pub fn list() {
    match confy::load::<SupremeConfig>("supreme") {
        Ok(t) => println!("{:#?}", t),
        Err(e) => println!("{:?}", e),
    }
}

fn get() -> Result<SupremeConfig, confy::ConfyError> {
    confy::load("supreme")
}

pub fn set(node_installer: NodeInstaller) -> Result<(), confy::ConfyError> {
    let supreme = SupremeConfig { node_installer };

    confy::store("supreme", supreme)?;
    list();

    Ok(())
}
