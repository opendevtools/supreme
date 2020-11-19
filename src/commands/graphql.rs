use crate::utils::{helpers, template};
use colored::*;
use helpers::Result;
use include_dir_macro::include_dir;
use serde_json::json;

pub fn run(output_path: String) -> Result<()> {
    // Create directory
    template::create_dir(&output_path)?;

    template::render_dir(
        include_dir!("src/templates/graphql"),
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

Start the app by running:

* {dev}
    ",
        title = "GraphQL setup completed".green(),
        output_path = &output_path.green(),
        install = "npm install".blue(),
        dev = "npm run dev".blue()
    );

    Ok(())
}
