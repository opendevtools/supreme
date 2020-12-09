use crate::utils::{helpers, npm, progressbar::Spinner, template};
use colored::*;
use helpers::Result;
use serde_json::json;
use std::fs;

pub fn git() -> Result<()> {
    template::render_file(include_str!("../templates/.gitignore"), ".gitignore", None)?;

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

    npm::install_dev("husky");
    npm::install_dev("pretty-quick");

    template::render_file(include_str!("../templates/.huskyrc"), ".huskyrc", None)?;

    spinner.success("Husky setup complete");

    Ok(())
}

pub fn prettier() -> Result<()> {
    let spinner = Spinner::new();

    spinner.set_message("Installing dependencies");

    npm::install_dev("prettier");

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

    npm::install_dev("jest");
    npm::install_dev("jest-watch-typeahead");

    template::render_file(
        include_str!("../templates/jest.config.js"),
        "jest.config.js",
        None,
    )?;

    spinner.success("Jest setup complete");

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

    npm::install_dev("@iteam/config");

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
