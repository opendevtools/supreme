use crate::utils::{helpers, template};
use helpers::Result;
use serde_json::json;
use std::fs;

pub fn run(no_npm: bool) -> Result<()> {
    fs::create_dir_all(".github/workflows")?;

    template::render(
        include_str!("../templates/github_actions/release.yml"),
        ".github/workflows/release.yml",
        Some(json!({ "no_npm": no_npm })),
    )?;
    template::render(
        include_str!("../templates/github_actions/pr_check.yml"),
        ".github/workflows/pr_check.yml",
        None,
    )?;
    template::render(
        include_str!("../templates/github_actions/.releaserc"),
        ".releaserc",
        Some(json!({ "no_npm": no_npm })),
    )?;

    Ok(())
}
