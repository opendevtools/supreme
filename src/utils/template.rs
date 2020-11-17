use super::helpers::Result;
use dialoguer::{theme::ColorfulTheme, Confirm};
use handlebars::Handlebars;
use serde_json::{json, Value};
use std::{collections, fs, path, str};

fn write_file(output_path: &str, output: &str) -> Result<()> {
    if fs::metadata(output_path).is_ok() {
        let proceed = Confirm::with_theme(&ColorfulTheme::default())
            .default(false)
            .with_prompt(format!("File name {} exists, overwrite?", output_path))
            .interact()?;

        if !proceed {
            return Ok(());
        }
    }

    fs::write(output_path, output)?;

    Ok(())
}

pub fn create(template: &str, data: &Value) -> String {
    Handlebars::new().render_template(template, &data).unwrap()
}

pub fn render_file(template: &str, output_path: &str, data: Option<Value>) -> Result<()> {
    let data = match data {
        Some(d) => d,
        None => json!({}),
    };
    let output = create(template, &data);

    write_file(output_path, &output)?;

    Ok(())
}

pub fn render_dir(
    folder: collections::HashMap<&path::Path, &[u8]>,
    output_path: &str,
    data: &Value,
) -> Result<()> {
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
