use anyhow::Error;
use serde_json::{from_str, to_string};

use crate::{
    models::{
        action_unit::ActionUnit,
        command::{AddCommand, CreateFile, DeleteFile, ReadFile},
    },
    store::filesystem::{
        create_command_file, delete_command_file, get_command_file, get_file_content, write_to_file,
    },
};

pub fn create_file(command: CreateFile) -> Result<(), Error> {
    create_command_file(&command.filename);
    Ok(())
}

pub fn delete_file(command: DeleteFile) -> Result<(), Error> {
    delete_command_file(&command.filename);
    Ok(())
}

pub fn display_file(command: ReadFile) -> Result<(), Error> {
    let f = get_command_file(&command.filename);
    let content = get_file_content(f);
    let result = from_str::<ActionUnit>(&content)?;

    println!("{}", result);

    Ok(())
}

pub fn add_command_to_file(command: AddCommand) -> Result<(), Error> {
    let f = get_command_file(&command.filename);
    let unit = ActionUnit::new(command.body, command.path);

    let serialized_unit = to_string(&unit)?;

    write_to_file(f, serialized_unit);

    Ok(())
}
