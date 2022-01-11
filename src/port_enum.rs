use std::process::Command;

pub fn run() {
    Command::new("sudo")
        .args(&[
            "masscan",
            "-p1-65535",
            "-iL",
            "ip.poseidon",
            "--max-rate",
            "2000",
            "-oG",
            "open_ports.poseidon",
        ])
        .spawn()
        .unwrap()
        .wait()
        .expect("masscan failed");
}
