use crate::utils::{
    helpers::{self, packages_to_strings},
    node,
    project::{Project, ProjectType},
};
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

    node::uninstall(&packages_to_strings(&["husky", "pretty-quick"]));

    Ok(())
}

pub fn prettier() -> Result<()> {
    fs::remove_file(".prettierrc")?;
    node::uninstall(&packages_to_strings(&["prettier"]));

    Ok(())
}

pub fn jest() -> Result<()> {
    let project = Project::new(None);
    fs::remove_file("jest.config.js")?;

    let jest = match project.project_type {
        ProjectType::ReScript => "@glennsl/bs-jest",
        ProjectType::JavaScript => "jest",
        ProjectType::Rust => panic!("Jest won't work in a Rust project"),
    };

    node::uninstall(&packages_to_strings(&[
        jest,
        "jest-watch-typeahead",
        "is-ci-cli",
    ]));

    node::remove_scripts(vec!["test", "test:ci", "test:watch"])?;

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

    node::uninstall(&packages_to_strings(&["@iteam/config"]));

    fs::remove_file("config.json")?;

    Ok(())
}

pub fn graphql_codegen() -> Result<()> {
    node::uninstall(&packages_to_strings(&[
        "graphql",
        "@graphql-codegen/cli",
        "@graphql-codegen/introspection",
        "@graphql-codegen/typescript",
        "@graphql-codegen/typescript-resolvers",
    ]));

    Ok(())
}
