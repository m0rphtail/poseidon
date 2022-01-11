use colored::*;
use std::env;
use std::path::{Path, PathBuf};

fn find_it<P>(exe_name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&exe_name);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            })
            .next()
    })
}

fn check(exe_name: &str) {
    match find_it(exe_name) {
        Some(_) => println!(
            "✔️ {}",
            exe_name.green().bold()
        ),
        None => println!(
            "❌ {} {}",
            "not installed".red().bold(),
            exe_name.red().bold()
        ),
    }
}

pub fn run() {
    println!("checking dependancies...");
    check("amass");
    check("massdns");
    check("masscan");
}
