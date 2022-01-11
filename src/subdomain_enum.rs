use itertools::Itertools;
use regex::Regex;
use spinners::{Spinner, Spinners};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn run(domain: &str) {
    get_subdomains(domain);
    resolve_subdomains();
    get_ip();
}

fn get_subdomains(domain: &str) {
    let sp = Spinner::new(&Spinners::Earth, "finding subdomains".into());
    let amass = Command::new("amass")
        .args(&["enum", "--passive", "-d"])
        .arg(domain)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut f = File::create("all_subdomains.poseidon").expect("unable to create file");
    io::copy(&mut amass.stdout.unwrap(), &mut f).expect("unable to write file");
    sp.stop();
    // print!("");
}

fn resolve_subdomains() {
    if !Path::new("resolvers.txt").exists() {
        let mut r = File::create("resolvers.txt").expect("resolvers.txt file not found");
        let resolvers: &str = include_str!("../massdns_resolvers.txt");
        r.write_all(resolvers.as_bytes())
            .expect("failed to create resolvers file");
    }
    Command::new("massdns")
        .args(&[
            "-r",
            "resolvers.txt",
            "-t",
            "A",
            "-o",
            "S",
            "all_subdomains.poseidon",
            "-w",
            "resolved_subdomains.poseidon",
        ])
        .spawn()
        .unwrap()
        .wait()
        .expect("failed to run massdns");
}

fn get_ip() {
    let file = File::open("resolved_subdomains.poseidon").expect("unable to open file");
    let file = BufReader::new(file);
    let mut data: Vec<String> = Vec::new();
    let mut ip_list: Vec<String> = Vec::new();

    for line in file.lines() {
        let line = line.expect("unable to read file");
        data.push(line.parse().unwrap());
    }

    let ip_regex = Regex::new(r"\b((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(\.|$)){4}\b").unwrap();

    for i in data {
        if ip_regex.is_match(&i) {
            ip_list.push(i.split(' ').last().unwrap().to_string());
        }
    }
    let ip_list: Vec<_> = ip_list.into_iter().unique().collect();
    let mut f = File::create("ip.poseidon").expect("Unable to create file");
    for i in ip_list {
        writeln!(f, "{}", i).unwrap();
    }

    println!("Created list of valid ip addresses")
}
