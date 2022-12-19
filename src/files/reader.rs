use std::{
    collections::HashMap,
    fs::{self, ReadDir},
    os::unix::prelude::PermissionsExt,
};

use colored::Colorize;

pub struct ReadResult {
    pub path: String,

    //all items<file name, permissions>
    pub all_items: HashMap<String, u32>,

    //all dirs
    pub dirs: Vec<String>,

    //all files
    pub files: Vec<String>,

    //symlinks
    pub symlinks: Vec<String>,
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

    fn sort(rres: &mut Self) {
        rres.dirs.sort();
        rres.files.sort();
        rres.symlinks.sort();
    }
}

pub fn read_all(path: &impl ToString) -> Result<ReadResult, std::io::Error> {
    let mut rres: ReadResult = ReadResult::new(path);
    let readdir = match get_readdir(path) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    match get_files(&mut rres, readdir) {
        Some(s) => {
            println!("one or more errors encountered");
            Some(s)
        }
        None => None,
    };
    //sort rres
    ReadResult::sort(&mut rres);

    return Ok(rres);
}

fn get_readdir(path: &impl ToString) -> Result<ReadDir, std::io::Error> {
    let entries: ReadDir = match fs::read_dir(path.to_string()) {
        Ok(v) => v,
        Err(e) => {
            return Err(e);
        }
    };
    return Ok(entries);
}

fn get_files(rr: &mut ReadResult, entries: ReadDir) -> Option<String> {
    //lets try listing files in directory
    for d in entries {
        if d.is_err() {
            continue;
        }
        //what if instead of cloning we use the name?

        let ent = d.expect("this shouldn't be printed");
        let file_name = ent.file_name();
        let name = match file_name.to_str() {
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

pub fn u32_perm_to_str(perm: &u32) -> String {
    let perm_u32: u32 = perm.clone();
    let mut last: String = String::new();
    let it_on: String = format!("{perm_u32:b}").chars().skip(4).collect();

    let mut out: String = String::new();

    for s in it_on.chars() {
        if last.len() < 3 {
            last.push(s);
            if last.len() < 3 {
                continue;
            }
        }
        //convert string of binary characters to an interger and append it to the permissions
        //string
        out.push_str(
            &u8::from_str_radix(&last, 2)
                .expect("this should always be an int")
                .to_string(),
        );
        last.clear();
    }
    return out;
}
