use crate::utils::{helpers, node};
use helpers::Result;
use std::fs;

pub fn git() -> Result<()> {
    fs::remove_file(".gitignore")?;

    Ok(())
}

pub fn nvm() -> Result<()> {
    fs::remove_file(".nvmrc")?;

    Ok(())
}

pub fn husky() -> Result<()> {
    fs::remove_file(".huskyrc")?;
    node::uninstall("husky pretty-quick");

    Ok(())
}

pub fn prettier() -> Result<()> {
    fs::remove_file(".prettierrc")?;
    node::uninstall("prettier");

    Ok(())
}

pub fn jest() -> Result<()> {
    fs::remove_file("jest.config.js")?;
    node::uninstall("jest jest-watch-typeahead");

    Ok(())
}

pub fn config() -> Result<()> {
    let is_typescript = fs::metadata("tsconfig.json").is_ok();
    let folder = if fs::metadata("./src").is_ok() {
        "src"
    } else {
        "lib"
    };

    if is_typescript {
        fs::remove_file(format!("{}/config.ts", folder))?;
    } else {
        fs::remove_file(format!("{}/config.js", folder))?;
    }

    node::uninstall("@iteam/config");
    fs::remove_file("config.json")?;

    Ok(())
}

pub fn graphql_codegen() -> Result<()> {
    node::uninstall("graphql @graphql-codegen/{cli,introspection,typescript,typescript-resolvers}");

    Ok(())
}
