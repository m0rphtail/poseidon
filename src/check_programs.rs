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
        Some(path) => println!("({:?}) {} already installed! ✔️", path, exe_name),
        None => println!("{} not installed ❌", exe_name),
    }
}

pub fn run() {
    check("amass");
    check("massdns");
    check("masscan");
}
