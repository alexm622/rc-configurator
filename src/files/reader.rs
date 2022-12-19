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
    let as_bin = format!("{perm_u32:b}");
    let mut last: String = String::from("");
    let mut all: u8 = 0;
    let mut group: u8 = 0;
    let mut owner: u8 = 0;
    let mut special: u8 = 0;
    let mut iter: u8 = 0;
    let mut dbg_str = "".to_owned();
    let it_on: String = as_bin.chars().skip(4).collect();
    for s in it_on.chars() {
        dbg_str.push(s);
        if last.len() < 3 {
            last.push(s);
            if iter != 3 && last.len() != 3 {
                continue;
            }
        }
        match iter {
            0 => {
                special = u8::from_str_radix(&last, 2).expect("all wasn't an int?");
            }
            1 => {
                owner = u8::from_str_radix(&last, 2).expect("all wasn't an int?");
            }
            2 => {
                group = u8::from_str_radix(&last, 2).expect("all wasn't an int?");
            }
            3 => {
                all = u8::from_str_radix(&last, 2).expect("all wasn't an int?");
            }
            _ => {
                println!(
                    "{}{}{}{}",
                    "somehow got a value of iter = ".red(),
                    iter.to_string().yellow(),
                    " for last value: ".red(),
                    last.yellow()
                );
            }
        };
        last.clear();
        iter += 1;
    }
    assert!(dbg_str == it_on);
    return format!("{}{}{}{}", special, owner, group, all);
}
