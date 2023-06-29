use crate::utils::{
    helpers,
    project::{Project, ProjectType},
    template,
};
use helpers::Result;
use serde_json::json;

pub fn run(no_npm: bool, project_type: Option<ProjectType>) -> Result<()> {
    let project = Project::new(project_type);

    project.log();

    let data = json!({ "name": env!("CARGO_PKG_NAME"), "noNpm": !no_npm });

    template::render_dir(project.directory(), ".github/workflows", &data)?;
    template::render_file(project.release_config(), ".releaserc.json", Some(&data))?;

    Ok(())
}
