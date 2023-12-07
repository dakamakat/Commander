mod models;
mod parser;
mod store;

use anyhow::Result;
use clap::Parser;
use models::command::{ActionSubcommand, ActionType, FileSubcommand};
use std::process::Command;
use store::filesystem::{create_command_file, delete_command_file, read_command_file};

fn main() -> Result<()> {
    let val = models::command::ActionUnit::parse();

    match val.action_type {
        ActionType::File(val) => match val.command {
            FileSubcommand::Create(val) => return create_command_file(val),
            FileSubcommand::Delete(val) => return delete_command_file(val),
            FileSubcommand::Read(val) => return read_command_file(val),
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
    Ok(())
}
