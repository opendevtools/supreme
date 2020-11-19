use crate::utils::{helpers, template};
use colored::*;
use helpers::Result;
use include_dir_macro::include_dir;
use serde_json::json;
use std::fs;

pub fn run(output_path: String) -> Result<()> {
    // Create directory
    fs::create_dir_all(&output_path)?;

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

Start the app by opening two terminal tabs and
running the following commands:

* {dev} (start compiler)
    ",
        title = "GraphQL setup completed".green(),
        output_path = &output_path.green(),
        install = "npm install".blue(),
        dev = "npm run dev".blue()
    );

    Ok(())
}
