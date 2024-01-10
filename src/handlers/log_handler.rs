#![allow(dead_code)]

use std::{
    io::{stdout, BufWriter, Write},
    process::Output,
    str::from_utf8,
};

use anyhow::{Context, Result};

pub fn write_to_console(str: &String) -> Result<()> {
    let stdout = stdout();

    let mut handle = BufWriter::new(stdout);

    writeln!(handle, "{}", str).with_context(|| "could not print line")?;

    Ok(())
}

pub fn log_output(output: Output) -> Result<()> {
    let mut log = String::new();

    let utf8 = from_utf8(&output.stdout).with_context(|| "got non UTF-8 data from git")?;

    log.push_str(utf8);

    println!("command output:\n{0}", log);

    Ok(())
}
