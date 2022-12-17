use colored::Colorize;

use users::get_current_gid;

use clap::Parser;

use std::fs::{self, DirEntry, ReadDir};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    //run without root
    #[arg(short, long)]
    no_root: bool,
}

const DEBUG: bool = true;

fn main() {
    //check to see if root
    let curr_uid = get_current_gid();
    if curr_uid != 0 && !DEBUG {
        println!("{}", "WARNING!!!".red());
        println!("you should only run this script as root");
        println!(
            "if you really think you know what you're doing then run this with the argument {}.",
            "--no-root".yellow()
        );
        return;
    }
    let args = Args::parse();
    if args.no_root {
        println!("ignoring root");
    }
    //lets try listing files in directory
    let entries: ReadDir = match fs::read_dir(".") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", format!("ERROR: {:?}", e).red());
            return;
        }
    };
    let mut dirs: Vec<String> = Vec::new();
    for d in entries {
        if d.is_err() {
            continue;
        }
        //what if instead of cloning we use the name?

        let ent = d.expect("this shouldn't be printed");
        let name = ent.file_name();
        let ftype = ent.file_type().expect("unknown file type?");
        if ftype.is_dir() {
            dirs.push(name.into_string().expect("string name failed"));
        }
    }

    for s in dirs {
        println!("{}", s.blue());
    }
}
