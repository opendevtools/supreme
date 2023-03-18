use crate::utils::{
    helpers::{self, packages_to_strings},
    node,
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
    node::install_dev(&packages_to_strings(&["husky", "pretty-quick"]));

    println!(
        "\n{cmd} - Initialize husky\n",
        cmd = "npx husky-init".blue(),
    );

    Ok(())
}

pub fn prettier() -> Result<()> {
    node::install_dev(&packages_to_strings(&["prettier"]));

    template::render_file(
        include_str!("../templates/.prettierrc"),
        ".prettierrc",
        None,
    )?;

    Ok(())
}

pub fn jest() -> Result<()> {
    let project = Project::new(None);

    project.log();

    let jest = match project.project_type {
        ProjectType::ReScript => "@glennsl/bs-jest",
        ProjectType::JavaScript => "jest",
        ProjectType::Rust => panic!("Jest won't work in a Rust project"),
    };

    let pkgs = vec![jest, "jest-watch-typeahead", "is-ci-cli"];

    node::install_dev(&packages_to_strings(&pkgs));

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

    println!(
        "\nNew commands added
* {test} - Run tests in either CI mode or watch mode in dev
* {test_ci} - CI mode runs only if CI environment variable is set, uses is-ci-cli
* {test_watch} - Run tests in watch mode
    ",
        test = "test".blue(),
        test_ci = "test:ci".blue(),
        test_watch = "test:watch".blue()
    );

    if let ProjectType::ReScript = project.project_type {
        println!(
            "\nAdd this to bsconfig.json:

\"bs-dev-dependencies\": [\"@glennsl/bs-jest\"],
\"sources\": [
  {{
    \"dir\": \"src\"
  }},
  {{
    \"dir\": \"__tests__\",
    \"type\": \"dev\"
  }}
]
    "
        )
    }

    Ok(())
}

pub fn config() -> Result<()> {
    let is_typescript = fs::metadata("tsconfig.json").is_ok();
    let folder = if fs::metadata("./src").is_ok() {
        "src"
    } else {
        "lib"
    };

    node::install_dev(&packages_to_strings(&["@iteam/config"]));

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

    Ok(())
}

pub fn graphql_codegen() -> Result<()> {
    let pkgs = vec![
        "graphql",
        "@graphql-codegen/cli",
        "@graphql-codegen/introspection",
        "@graphql-codegen/typescript",
        "@graphql-codegen/typescript-resolvers",
    ];

    node::install_dev(&packages_to_strings(&pkgs));

    println!(
        "\n* Run {command} to setup the project configuration\n",
        command = "npx graphql-codegen init".blue()
    );

    Ok(())
}

pub fn tailwind() -> Result<()> {
    let pkgs = vec!["tailwindcss", "postcss", "autoprefixer"];

    node::install_dev(&packages_to_strings(&pkgs));

    template::render_file(
        include_str!("../templates/tailwind/postcss.config.js"),
        "postcss.config.js",
        None,
    )?;

    helpers::spawn_command("npx", &["tailwindcss", "init"])?;

    println!(
        "Create a CSS file with the following content to include Tailwinds styling.

{tailwind} base;
{tailwind} components;
{tailwind} utilities;
    ",
        tailwind = "@tailwind".blue()
    );

    Ok(())
}
