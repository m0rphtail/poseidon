use clap::Parser;

mod banner;
mod check_programs;
mod port_enum;
mod subdomain_enum;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    domain: String,
}

fn main() {
    let args = Args::parse();
    banner::run();
    check_programs::run();
    subdomain_enum::run(&args.domain);
    port_enum::run();
}
