use colored::*;

use crate::config::NodeInstaller;

pub struct Message {
    package: ColoredString,
    package_manager: NodeInstaller,
}

impl Message {
    pub fn new(package: &str) -> Self {
        Self {
            package: package.blue(),
            package_manager: NodeInstaller::default(),
        }
    }

    pub fn install(&self, msg: &str) {
        println!(
            "⌛ {} {} using {}",
            msg,
            self.package,
            self.package_manager.to_string().green()
        );
    }

    pub fn success(&self, msg: &str) {
        println!("{} {} {}", "✓".green(), msg, self.package);
    }
}
