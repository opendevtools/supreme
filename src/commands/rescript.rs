use crate::utils::helpers;
use colored::*;
use copy_dir::copy_dir;
use handlebars::Handlebars;
use helpers::Result;
use serde_json::json;
use std::fs;

fn update_file(dir: &str, filename: &str) -> Result<()> {
    let reg = Handlebars::new();
    let file = &fs::read(format!("{}/{}", dir, filename))?;
    let file: String = String::from_utf8_lossy(file).parse()?;
    let output = reg.render_template(&file, &json!({ "name": dir }))?;

    fs::write(&format!("{}/{}", dir, filename), output)?;

    Ok(())
}

pub fn run(name: String) -> Result<()> {
    let rescript_template = helpers::application_dir("src/templates/rescript")?;

    copy_dir(rescript_template, &name)?;

    // Update files with template names
    update_file(&name, "package.json")?;
    update_file(&name, "bsconfig.json")?;
    update_file(&name, "README.md")?;
    update_file(&name, "public/index.html")?;

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
