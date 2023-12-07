use std::process::Command;

use clap::Parser;

mod models;
mod parser;
mod store;

fn main() {
    let val = models::command::ActionUnit::parse();

    match val.action_type {
        models::command::ActionType::File(val) => match val.command {
            models::command::FileSubcommand::Create(_val) => todo!(),
            models::command::FileSubcommand::Delete => todo!(),
            models::command::FileSubcommand::Show => todo!(),
        },
        models::command::ActionType::Action(val) => match val.command {
            models::command::ActionSubcommand::Invoke(val) => {
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
