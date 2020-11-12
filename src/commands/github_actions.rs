use crate::utils::{helpers, template};
use helpers::Result;
use serde_json::json;
use std::fs;

pub fn run(no_npm: bool) -> Result<()> {
    let has_bs_config = fs::metadata("bsconfig.json").is_ok();

    if has_bs_config {
        println!("Found ReScript project");
    }

    fs::create_dir_all(".github/workflows")?;

    template::render(
        include_str!("../templates/github_actions/release.yml"),
        ".github/workflows/release.yml",
        Some(json!({ "noNpm": no_npm, "isRescript": has_bs_config })),
    )?;
    template::render(
        include_str!("../templates/github_actions/pr_check.yml"),
        ".github/workflows/pr_check.yml",
        Some(json!({ "isRescript": has_bs_config })),
    )?;
    template::render(
        include_str!("../templates/github_actions/.releaserc"),
        ".releaserc",
        Some(json!({ "noNpm": no_npm })),
    )?;

    Ok(())
}
