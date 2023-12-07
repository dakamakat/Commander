mod models;
mod parser;
mod store;

use clap::Parser;
use models::command::{ActionSubcommand, ActionType, FileSubcommand};
use std::process::Command;

fn main() {
    let val = models::command::ActionUnit::parse();

    match val.action_type {
        ActionType::File(val) => match val.command {
            FileSubcommand::Create(_val) => todo!(),
            FileSubcommand::Delete => todo!(),
            FileSubcommand::Show => todo!(),
        },
        ActionType::Action(val) => match val.command {
            ActionSubcommand::Invoke(val) => {
                println!("body: {0} , path: {1} \n", val.body, val.path);

                let output = Command::new(val.body)
                    .current_dir(val.path)
                    .output()
                    .expect("Failed to execute command");

                let mut log = String::new();

                log.push_str(match std::str::from_utf8(&output.stdout) {
                    Ok(val) => val,
                    Err(_) => panic!("got non UTF-8 data from git"),
                });

                println!("command output:\n{0}", log,)
            }
        },
    }
}
