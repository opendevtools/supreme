use colored::*;

pub fn success(msg: &str) {
    println!("{check} {msg}", check = "✔".green(), msg = msg);
}
