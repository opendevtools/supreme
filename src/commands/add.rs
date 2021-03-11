use crate::utils::{
    helpers, node,
    progressbar::Spinner,
    project::{Project, ProjectType},
    template,
};
use colored::*;
use helpers::Result;
use serde_json::json;
use std::collections::HashMap;
use std::fs;

pub fn git(project_type: Option<ProjectType>) -> Result<()> {
    let project = Project::new(project_type);

    project.log();

    template::render_file(project.gitignore(), ".gitignore", None)?;

    Ok(())
}

pub fn nvm() -> Result<()> {
    let output = helpers::run_command("node", &["-v"]);
    let output = String::from_utf8(output.stdout)?;
    let version = output.trim_end();

    template::render_file(
        include_str!("../templates/.nvmrc"),
        ".nvmrc",
        Some(&json!({ "version": version })),
    )?;

    Ok(())
}

pub fn husky() -> Result<()> {
    let spinner = Spinner::new();

    spinner.set_message("Installing dependencies");

    node::install_dev("husky pretty-quick");

    template::render_file(include_str!("../templates/.huskyrc"), ".huskyrc", None)?;

    spinner.success("Husky setup complete");

    Ok(())
}

pub fn prettier() -> Result<()> {
    let spinner = Spinner::new();

    spinner.set_message("Installing dependencies");

    node::install_dev("prettier");

    template::render_file(
        include_str!("../templates/.prettierrc"),
        ".prettierrc",
        None,
    )?;

    spinner.success("Prettier setup complete");

    Ok(())
}

pub fn jest() -> Result<()> {
    let spinner = Spinner::new();

    spinner.set_message("Installing dependencies");

    node::install_dev("jest jest-watch-typeahead is-ci-cli");

    let mut scripts = HashMap::new();

    scripts.insert("test", "is-ci-cli test:ci test:watch");
    scripts.insert("test:ci", "jest");
    scripts.insert("test:watch", "jest --watch");

    node::add_scripts(scripts)?;

    template::render_file(
        include_str!("../templates/jest.config.js"),
        "jest.config.js",
        None,
    )?;

    spinner.success("Jest setup complete");

    println!(
        "
New commands added
* {test} - Run tests in either CI mode or watch mode in dev
* {test_ci} - CI mode runs only if CI environment variable is set, uses is-ci-cli
* {test_watch} - Run tests in watch mode
    ",
        test = "test".blue(),
        test_ci = "test:ci".blue(),
        test_watch = "test:watch".blue()
    );

    Ok(())
}

pub fn config() -> Result<()> {
    let spinner = Spinner::new();
    let is_typescript = fs::metadata("tsconfig.json").is_ok();
    let folder = if fs::metadata("./src").is_ok() {
        "src"
    } else {
        "lib"
    };

    spinner.set_message("Installing dependencies");

    node::install_dev("@iteam/config");

    if is_typescript {
        template::render_file(
            include_str!("../templates/config/config.ts"),
            &format!("{}/config.ts", folder),
            None,
        )?;
    } else {
        template::render_file(
            include_str!("../templates/config/config.js"),
            &format!("{}/config.js", folder),
            None,
        )?;
    }

    template::render_file(
        include_str!("../templates/config/config.json"),
        "config.json",
        None,
    )?;

    spinner.success("Config setup complete");

    Ok(())
}

pub fn graphql_codegen() -> Result<()> {
    let spinner = Spinner::new();

    spinner.set_message("Installing dependencies");

    node::install_dev(
        "graphql @graphql-codegen/{cli,introspection,typescript,typescript-resolvers}",
    );

    spinner.success("GraphQL Codegen installed");

    println!(
        "
* Run {command} to setup the project configuration
    ",
        command = "npx graphql-codegen init".blue()
    );

    Ok(())
}
