use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct ActionUnit {

    /// Actual command
    pub body: String,

    /// Path to execute command
    pub path: String,
}
