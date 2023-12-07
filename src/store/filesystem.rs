use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Write},
};

use anyhow::{Context, Result as AnyResult};

use crate::models::command::{CreateFile, DeleteFile, ReadFile};

pub fn create_command_file(command: CreateFile) -> AnyResult<()> {
    let mut f = File::create(&command.filename).with_context(|| {
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
    fs::remove_file(&command.filename).with_context(|| {
        format!(
            "Error trying to delete file with name {}",
            &command.filename
        )
    })?;
    Ok(())
}

pub fn read_command_file(command: ReadFile) -> AnyResult<()> {
    let f = File::open(&command.filename)
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
