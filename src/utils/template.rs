use super::helpers::Result;
use dialoguer::{theme::ColorfulTheme, Confirm};
use handlebars::Handlebars;
use serde_json::{json, Value};
use std::fs;

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

pub fn render(template: &str, output_path: &str, data: Option<Value>) -> Result<()> {
    let reg = Handlebars::new();
    let data = match data {
        Some(d) => d,
        None => json!({}),
    };
    let output = reg.render_template(template, &data)?;

    write_file(output_path, &output)?;

    Ok(())
}