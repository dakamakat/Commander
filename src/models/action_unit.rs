use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionUnit {
    /// Body of the command
    pub body: String,
    /// Path to execute command
    pub path: String,
}

impl ActionUnit {
    pub fn new(body: String, path: String) -> Self {
        Self { body, path }
    }
}

impl Display for ActionUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.body, self.path)
    }
}
