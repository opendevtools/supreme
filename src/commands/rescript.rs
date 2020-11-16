use crate::utils::{helpers, template};
use colored::*;
use helpers::Result;
use include_dir_macro::include_dir;
use serde_json::json;
use std::{fs, path, str};

pub fn run(name: String) -> Result<()> {
    // Create public and src folder
    fs::create_dir_all(format!("{}/public", &name))?;
    fs::create_dir_all(format!("{}/src", &name))?;

    let template = include_dir!("src/templates/rescript");

    // Loop through all files in template
    for key in template.keys() {
        let file = template
            .get(path::Path::new(key))
            .and_then(|entry| str::from_utf8(*entry).ok())
            .unwrap();
        let output = template::create(file, &json!({ "name": &name }));

        fs::write(format!("{}/{}", &name, &key.to_string_lossy()), output)?;
    }

    println!(
        "
{title}
--------------------
Install dependencies

* cd {name}
* {install}

Start the app by opening two terminal tabs and
running the following commands:

* {compiler} (start compiler)
* {server} (start development server on port 3000)
    ",
        title = "ReScript setup completed".green(),
        name = &name.green(),
        compiler = "npm start".blue(),
        server = "npm run server".blue(),
        install = "npm install".blue(),
    );

    Ok(())
}
