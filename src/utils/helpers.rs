use std::{env, fs, io, path, process};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn run_command(cmd: &str, arg: &[&str]) -> process::Output {
    process::Command::new(cmd)
        .args(arg)
        .output()
        .expect("Failed to execute command")
}

pub fn install_dev(pkg: &str) -> process::Output {
    run_command("npm", &["install", "--save-exact", "--save-dev", pkg])
}

pub fn application_root_dir() -> std::result::Result<path::PathBuf, io::Error> {
    if let Some(manifest_dir) = env::var_os("CARGO_MANIFEST_DIR") {
        return Ok(path::PathBuf::from(manifest_dir));
    }

    let mut exe = fs::canonicalize(env::current_exe()?)?;

    // Modify in-place to avoid an extra copy.
    if exe.pop() {
        return Ok(exe);
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "Failed to find an application root",
    ))
}

/// Same as `application_root_dir`, but extends the root directory with the given path.
pub fn application_dir<P>(path: P) -> std::result::Result<path::PathBuf, io::Error>
where
    P: AsRef<path::Path>,
{
    Ok(application_root_dir()?.join(path))
}
