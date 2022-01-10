// use colored::*;
use figlet_rs::FIGfont;

pub fn run() {
    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font.convert("poseidon");
    println!("{}", figure.unwrap());
    println!("{}", env!("CARGO_PKG_VERSION"));
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));
    println!("====================================");
}
