use super::helpers::Result;
use dialoguer::{theme::ColorfulTheme, Confirm};
use handlebars::Handlebars;
use serde_json::{json, Value};
use std::io::ErrorKind;
use std::{collections, fs, path, str};

fn write_file(output_path: &str, output: &str) -> Result<()> {
    match fs::metadata(output_path) {
        Ok(_) => {
            let proceed = Confirm::with_theme(&ColorfulTheme::default())
                .default(false)
                .with_prompt(format!("File name {} exists, overwrite?", output_path))
                .interact()?;

            if !proceed {
                return Ok(());
            }
        }
        Err(err) => {
            if let ErrorKind::NotFound = err.kind() {
                create_dir(output_path)?;
            }
        }
    }

    fs::write(output_path, output)?;

    Ok(())
}

pub fn create(template: &str, data: &Value) -> String {
    Handlebars::new().render_template(template, &data).unwrap()
}

pub fn render_file(template: &str, output_path: &str, data: Option<&Value>) -> Result<()> {
    let output = create(template, data.unwrap_or(&json!({})));

    write_file(output_path, &output)?;

    Ok(())
}

pub fn create_dir(output_path: &str) -> Result<()> {
    let path = path::Path::new(output_path);
    let parent_dir = path.parent().unwrap();
    fs::create_dir_all(parent_dir)?;

    Ok(())
}

pub fn render_dir(
    folder: collections::HashMap<&path::Path, &[u8]>,
    output_path: &str,
    data: &Value,
) -> Result<()> {
    fs::create_dir_all(output_path)?;

    for key in folder.keys() {
        let file = folder
            .get(path::Path::new(key))
            .and_then(|entry| str::from_utf8(*entry).ok())
            .unwrap();
        let output = create(file, data);

        write_file(
            &format!("{}/{}", output_path, &key.to_string_lossy()),
            &output,
        )?;
    }

    Ok(())
}
