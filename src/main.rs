use clap::Parser;

mod check_programs;
mod subdomain_enum;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    domain: String,
    // #[clap(short, long)]
    // project: String,
}

fn main() {
    let args = Args::parse();

    check_programs::run();
    // setup_project(&args.project);
    subdomain_enum::run(&args.domain);
}

// fn setup_project(folder: &str) {
//     Command::new("mkdir")
//         .args(&["-p", folder])
//         .spawn()
//         .expect("unable crate project directory");
// }
