use crate::utils::helpers;
use colored::*;
use handlebars::Handlebars;
use helpers::Result;
use include_dir_macro::include_dir;
use serde_json::json;
use std::fs;

pub fn run(name: String) -> Result<()> {
    let reg = Handlebars::new();
    fs::create_dir_all(&name)?;
    fs::create_dir_all(format!("{}/public", &name))?;
    fs::create_dir_all(format!("{}/src", &name))?;

    let hashmap = include_dir!("src/templates/rescript");

    for key in hashmap.keys() {
        let file = std::path::Path::new(key);
        let text = hashmap
            .get(file)
            .and_then(|entry| std::str::from_utf8(*entry).ok())
            .unwrap();
        let output = reg.render_template(&text, &json!({ "name": &name }))?;

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
