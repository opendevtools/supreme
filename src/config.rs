extern crate confy;

use serde::{Deserialize, Serialize};

#[derive(clap::ArgEnum, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum NodeInstaller {
    Npm,
    Yarn,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupremeConfig {
    pub node_installer: NodeInstaller,
}

impl ::std::default::Default for SupremeConfig {
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

pub fn get() -> Result<SupremeConfig, confy::ConfyError> {
    confy::load("supreme")
}

pub fn set(node_installer: NodeInstaller) -> Result<(), confy::ConfyError> {
    let supreme = SupremeConfig { node_installer };

    confy::store("supreme", supreme)?;
    list();

    Ok(())
}
