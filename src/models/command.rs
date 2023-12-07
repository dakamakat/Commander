use clap::{command, Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[command(
    author = "dakamakat", 
    version, 
    about = "Basic tool to execute commands in provided location", 
    long_about = None)]
 pub struct ActionUnit {
    /// Type of action ( e.g storage change , command exec and etc
    #[clap(subcommand)]
    pub action_type: ActionType,
}

#[derive(Debug , Subcommand)]
pub enum ActionType {
    /// Create , delete or show file commands
    File(FileCommand),
    /// Invoke commands
    Action(ActionCommand)
}

#[derive(Debug , Args)]
pub struct FileCommand {
    #[clap(subcommand)]
    pub command: FileSubcommand
}

#[derive(Debug , Args)]
pub struct ActionCommand {
    #[clap(subcommand)]
    pub command: ActionSubcommand
}


#[derive(Debug, Subcommand)]
pub enum FileSubcommand { 
    /// Create file with commands
    Create(CreateFile),
    /// Delete file with commands
    Delete,
    /// Show all content from file with commands
    Show
}

#[derive(Debug, Subcommand)]
pub enum ActionSubcommand { 
    /// Invoke Specified command
    Invoke(InvokeAction)
}

#[derive(Debug, Args)]
pub struct InvokeAction {
    /// Body of the command
    pub body: String,
    /// Path to execute command
    pub path: String,
}

#[derive(Debug, Args)]
pub struct CreateFile {
    /// Body of the command
    pub body: String,
}
