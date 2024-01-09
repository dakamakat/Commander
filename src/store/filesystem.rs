use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Write},
};

use anyhow::{Context, Ok, Result as AnyResult};

use crate::{
    constants::filesystem_constants::{
        DEFAULT_COMMANDS_DIRECTORY_NAME, DEFAULT_COMMANDS_DIRECTORY_PATH,
    },
    models::command::{CreateFile, DeleteFile, ReadFile},
};

pub fn on_init() -> AnyResult<()> {
    create_command_directory()
}

pub fn create_command_file(command: CreateFile) -> AnyResult<()> {
    let mut f = File::create(get_command_file_path(&command.filename)).with_context(|| {
        format!(
            "Error trying to create file with name {}",
            &command.filename
        )
    })?;

    f.write_all(command.body.as_bytes())
        .with_context(|| format!("Error trying to write file with content {}", &command.body))?;

    Ok(())
}

pub fn delete_command_file(command: DeleteFile) -> AnyResult<()> {
    fs::remove_file(get_command_file_path(&command.filename)).with_context(|| {
        format!(
            "Error trying to delete file with name {}",
            &command.filename
        )
    })?;
    Ok(())
}

pub fn read_command_file(command: ReadFile) -> AnyResult<()> {
    let f = File::open(get_command_file_path(&command.filename))
        .with_context(|| format!("could not find file with name `{}`", &command.filename))?;

    let reader = BufReader::new(f);

    for line in reader.lines() {
        let res =
            line.with_context(|| format!("could not read line with name `{}`", &command.filename))?;

        write_to_console(&res)?;
    }

    Ok(())
}

fn write_to_console(str: &String) -> AnyResult<()> {
    let stdout = io::stdout();

    let mut handle = io::BufWriter::new(stdout);

    writeln!(handle, "{}", str).with_context(|| format!("could not print line"))?;

    Ok(())
}

fn create_command_directory() -> AnyResult<()> {
    fs::create_dir_all(DEFAULT_COMMANDS_DIRECTORY_NAME).with_context(|| {
        format!(
            "Error trying to create directory with name {}",
            DEFAULT_COMMANDS_DIRECTORY_NAME
        )
    })?;

    Ok(())
}

fn get_command_file_path(filename: &String) -> String {
    let command_file_path = format!("{}/{}.txt", DEFAULT_COMMANDS_DIRECTORY_PATH, filename);

    command_file_path
}
