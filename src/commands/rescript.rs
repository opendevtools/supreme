use crate::utils::{helpers, template};
use colored::*;
use helpers::Result;
use include_dir_macro::include_dir;
use serde_json::json;

pub fn run(output_path: String) -> Result<()> {
    template::render_dir(
        include_dir!("src/templates/rescript"),
        &output_path,
        &json!({ "name": &output_path }),
    )?;

    println!(
        "
{title}
--------------------
Install dependencies

* cd {output_path}
* {install}

Start the app by opening two terminal tabs and
running the following commands:

* {compiler} (start compiler)
* {server} (start development server on port 3000)
    ",
        title = "ReScript setup completed".green(),
        output_path = &output_path.green(),
        compiler = "npm start".blue(),
        server = "npm run server".blue(),
        install = "npm install".blue(),
    );

    Ok(())
}
