mod actions;
mod constants;
mod handlers;
mod models;
mod store;

use actions::invoke::{invoke_file, invoke_unit};
use anyhow::Result;
use clap::Parser;
use handlers::file_handler::{add_command_to_file, create_file, delete_file, display_file};
use models::{
    action_unit::ActionUnit,
    command::{ActionSubcommand, ActionType, FileSubcommand},
};
use store::filesystem::on_init;

fn main() -> Result<()> {
    let _on_init = on_init();

    let val = models::command::Action::parse();

    match val.action_type {
        ActionType::File(val) => match val.command {
            FileSubcommand::Create(val) => return create_file(val),
            FileSubcommand::Delete(val) => return delete_file(val),
            FileSubcommand::Display(val) => return display_file(val),
            FileSubcommand::Add(val) => return add_command_to_file(val),
        },
        ActionType::Action(val) => match val.command {
            ActionSubcommand::Invoke(val) => invoke_file(val.filename)?,
            ActionSubcommand::DirectInvoke(val) => {
                invoke_unit(ActionUnit::new(val.body, val.path))?
            }
        },
    }
    Ok(())
}
