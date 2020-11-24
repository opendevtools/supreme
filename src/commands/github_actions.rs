use crate::utils::{helpers, template};
use helpers::Result;
use include_dir_macro::include_dir;
use serde_json::json;
use std::fs;

pub fn run(no_npm: bool) -> Result<()> {
    let has_bs_config = fs::metadata("bsconfig.json").is_ok();

    if has_bs_config {
        println!("Found ReScript project, adding build step");
    }

    fs::create_dir_all(".github/workflows")?;

    template::render_dir(
        include_dir!("src/templates/github_actions"),
        ".github/workflows",
        &json!({ "noNpm": no_npm, "isRescript": has_bs_config }),
    )?;

    template::render_file(
        include_str!("../templates/.releaserc"),
        ".releaserc",
        Some(json!({ "noNpm": no_npm })),
    )?;

    Ok(())
}
