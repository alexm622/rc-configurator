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

mod files;

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

    dirs.sort();
    files.sort();

    for s in dirs {
        println!("{}", s.blue());
    }
    for f in files {
        println!("{}", f.red());
    }
    for sl in symlinks {}
}
