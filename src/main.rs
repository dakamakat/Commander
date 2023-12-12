mod actions;
mod models;
mod parser;
mod store;

use actions::invoke::invoke;
use anyhow::Result;
use clap::Parser;
use models::command::{ActionSubcommand, ActionType, FileSubcommand};
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
            ActionSubcommand::Invoke(val) => invoke(val.body, val.path),
        },
    }
    Ok(())
}
