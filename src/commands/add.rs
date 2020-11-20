use crate::utils::{helpers, template};
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
        Some(json!({ "version": version })),
    )?;

    Ok(())
}

pub fn husky() -> Result<()> {
    helpers::install_dev("husky");
    helpers::install_dev("pretty-quick");

    template::render_file(include_str!("../templates/.huskyrc"), ".huskyrc", None)?;

    Ok(())
}

pub fn prettier() -> Result<()> {
    helpers::install_dev("prettier");

    template::render_file(
        include_str!("../templates/.prettierrc"),
        ".prettierrc",
        None,
    )?;

    Ok(())
}

pub fn jest() -> Result<()> {
    helpers::install_dev("jest");
    helpers::install_dev("jest-watch-typeahead");

    template::render_file(
        include_str!("../templates/jest.config.js"),
        "jest.config.js",
        None,
    )?;

    Ok(())
}

pub fn config() -> Result<()> {
    let is_typescript = fs::metadata("tsconfig.json").is_ok();
    let folder = if fs::metadata("./src").is_ok() {
        "src"
    } else {
        "lib"
    };

    helpers::install_dev("@iteam/config");

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
