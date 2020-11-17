use crate::utils::{helpers, template};
use helpers::Result;
use serde_json::json;

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
