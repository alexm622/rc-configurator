use std::error::Error;

use super::reader::read_all;

use colored::Colorize;

use crate::errors::GenericError;

use std::collections::HashMap;

pub struct RC {
    pub path: String,
    pub has_rc_conf_d: bool,
    pub data: Option<RcData>,
}

pub struct RcData {
    pub beginning_data: Option<String>,
    pub blocks: Vec<RcBlock>,
}

pub struct RcBlock {
    // the title of a block
    // this would describe with what the below items correspond to
    // this must be a single string with no spaces or be preappended with a hyphen
    pub title: Option<String>,
    pub contents: Option<Vec<String>>,
    pub num_contents: u32,

    // main description, this would be below the title of the block
    pub description: Option<String>,

    //mapped description
    // item number and desc. desc may be multiple lines
    pub mapped_description: HashMap<u32, String>,
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
            data: None,
        }
    }

    pub fn rc_conf_exists(&self) -> bool {
        std::path::Path::new(&self.path).exists()
    }

    pub fn read_rc_conf(&self) -> String {
        if self.rc_conf_exists() {
            "placeholder".to_owned()
        } else {
            "rc.conf does not exist".to_owned()
        }
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
