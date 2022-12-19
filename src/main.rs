use colored::Colorize;

use files::reader::read_all;
use users::get_current_gid;

use clap::Parser;

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
    let path = String::from("/etc/rc.d/");
    let rres = match read_all(&path) {
        Ok(v) => v,
        Err(e) => panic!("encountered error: {}", e),
    };

    for s in rres.dirs {
        println!("{}", s.blue());
    }
    for f in rres.files {
        println!("{}", f.red());
    }
    for sl in rres.symlinks {
        println!("{}", sl);
    }
}
