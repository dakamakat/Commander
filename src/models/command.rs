use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(
    author = "dakamakat", 
    version, 
    about = "Basic tool to execute commands in provided location", 
    long_about = None)]
pub struct ActionUnit {
    /// Actual command
    pub body: String,

    /// Path to execute command
    pub path: String,
}
