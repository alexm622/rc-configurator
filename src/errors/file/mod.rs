//not exist error
use std::{error::Error, fmt::Display};

use colored::Colorize;

#[derive(Debug)]
pub struct NotExistError {
    path: String,
}

impl NotExistError {
    fn new(path: &impl ToString) -> Self {
        NotExistError {
            path: path.to_string().to_owned(),
        }
    }
}

impl Error for NotExistError {}

impl Display for NotExistError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: String = format!(
            "{}{}{}",
            "File: ".red(),
            self.path.to_string().yellow(),
            "Was not found!".red()
        );
        write!(f, "{}", msg)
    }
}
