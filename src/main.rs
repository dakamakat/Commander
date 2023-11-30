use clap::Parser;
use std::process::Command;
use std::str;

mod models;

fn main() {
    let args = models::command::ActionUnit::parse();

    let output = Command::new(args.body)
        .current_dir(args.path)
        .output()
        .expect("Failed to execute command");

    let mut log = String::new();

    log.push_str(match str::from_utf8(&output.stdout) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data"),
    });

    println!("{0}", log)
}
