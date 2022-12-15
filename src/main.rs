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
    for e in entries {
        let ent = match e {
            Ok(v) => v,
            _ => continue,
        };
        match ent.file_type() {
            Ok(v) => {
                //this is stupid but its whatever rn
                if v.is_dir() {
                    println!("{}", ent.file_name().to_str().unwrap().blue());
                } else {
                    println!("{}", ent.file_name().to_str().unwrap());
                }
            }
            _ => {
                println!("cant access file_type for whatever reason");
                continue;
            }
        };
    }
}
