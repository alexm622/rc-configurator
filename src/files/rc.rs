use std::error::Error;

use super::reader::read_all;

use colored::Colorize;

use crate::errors::GenericError;

pub struct RC {
    pub path: String,
    pub has_rc_conf_d: bool,
}

impl RC {
    pub fn new(path: &impl ToString) -> Self {
        println!("creating new RC");
        RC {
            path: path.to_string().to_owned(),
            has_rc_conf_d: match has_rc_d() {
                Ok(v) => v,
                Err(_) => false,
            },
        }
    }

    pub fn rc_conf_exists(&self) -> bool {
        std::path::Path::new(&self.path).exists()
    }
}

fn has_rc_d() -> Result<bool, impl Error> {
    let path = "/etc/";
    let foldername = "rc.d";
    let dirconts = match read_all(&path) {
        Ok(v) => v,
        Err(e) => {
            print!("Error encountered: {}", e.to_string().red());
            return Err(GenericError::new(&e.to_string()));
        }
    };
    let folders = dirconts.dirs;
    Ok(folders.contains(&foldername.to_owned()))
}
