use colored::*;

pub struct Message {
    package: ColoredString,
}

impl Message {
    pub fn new(package: &str) -> Self {
        Self {
            package: package.blue(),
        }
    }

    pub fn install(&self, msg: &str, manager: &str) {
        println!("⌛ {} {} using {}", msg, self.package, manager.green());
    }

    pub fn success(&self, msg: &str) {
        println!("{} {} {}", "✓".green(), msg, self.package);
    }
}
