use clap::Parser;

mod check_programs;
mod subdomain_enum;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    domain: String,
    #[clap(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();

    check_programs::run();
    subdomain_enum::run(&args.domain, &args.output);
}
