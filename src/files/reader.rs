use std::{
    collections::HashMap,
    error::Error,
    fs::{self, ReadDir},
    os::unix::prelude::PermissionsExt,
};

pub struct ReadResult {
    path: String,

    //all items<file name, permissions>
    all_items: HashMap<String, u32>,

    //all dirs
    dirs: Vec<String>,

    //all files
    files: Vec<String>,

    //symlinks
    symlinks: Vec<String>,
}

impl ReadResult {
    fn new(path: &impl ToString) -> Self {
        ReadResult {
            path: path.to_string().to_owned(),
            all_items: HashMap::new(),
            dirs: Vec::new(),
            files: Vec::new(),
            symlinks: Vec::new(),
        }
    }
}

pub fn read_all(path: &impl ToString) -> Result<ReadResult, std::io::Error> {
    let res: ReadResult = ReadResult::new(path);
    return Ok(res);
}

fn get_readdir(path: &impl ToString) -> Result<ReadDir, std::io::Error> {
    let entries: ReadDir = match fs::read_dir("/etc/rc.d/") {
        Ok(v) => v,
        Err(e) => {
            return Err(e);
        }
    };
    return Ok(entries);
}

fn get_files(rr: &ReadResult, entries: ReadDir) -> Option<String> {
    //lets try listing files in directory
    for d in entries {
        if d.is_err() {
            continue;
        }
        //what if instead of cloning we use the name?

        let ent = d.expect("this shouldn't be printed");
        let name = match ent.file_name().to_str() {
            Some(v) => v,
            None => return Some("couldn't get name".to_owned()),
        };
        let perms = match ent.metadata() {
            Ok(v) => v.permissions().mode(),
            Err(e) => return Some(e.to_string()),
        };
        let ftype = match ent.file_type() {
            Ok(v) => v,
            Err(e) => return Some(e.to_string()),
        };
        //add to hashmap
        rr.all_items.insert(name.to_owned(), perms);
        if ftype.is_dir() {
            rr.dirs.push(name.to_owned());
        } else if ftype.is_file() {
            rr.files.push(name.to_owned());
        } else if ftype.is_symlink() {
            rr.symlinks.push(name.to_owned());
        }
    }
    return None;
}
