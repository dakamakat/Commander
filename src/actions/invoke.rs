use anyhow::{Error, Result};
use serde_json::from_str;

use std::process::{Command, Output};

use crate::{
    handlers::log_handler::log_output,
    models::action_unit::ActionUnit,
    store::filesystem::{get_command_file, get_file_content},
};

pub fn invoke_file(filename: String) -> Result<(), Error> {
    let f = get_command_file(&filename);
    let content = get_file_content(f);

    let unit = from_str::<ActionUnit>(&content)?;

    let output = execute(unit.body, unit.path);

    log_output(output)
}

pub fn invoke_unit(unit: ActionUnit) -> Result<()> {
    println!("{}", unit);

    let output = execute(unit.body, unit.path);

    log_output(output)
}

fn execute(body: String, path: String) -> Output {
    let output = Command::new(body)
        .current_dir(path)
        .output()
        .expect("Failed to execute command");

    output
}
