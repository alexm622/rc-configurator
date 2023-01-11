//define module structure

mod errors;
mod files;

use std::rc::Rc;

//begin
use colored::Colorize;

use files::reader::read_all;
use users::get_current_gid;

use clap::Parser;

use crate::files::rc::RC;

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
    println!("{}", "lets try reading the hashmap".yellow());
    for i in rres.all_items {
        println!(
            "{} - {}: {}",
            files::reader::u32_perm_to_str(&i.1),
            i.1.to_string().red(),
            i.0.to_string().yellow()
        );
    }

    let rs = RC::new(&"/etc/rc.conf");
    println!(
        "Does rc.conf exist: {}",
        match rs.rc_conf_exists() {
            true => "YES".green(),
            false => "NO".red(),
        }
    );

    //lets try reading rc.conf
    if rs.rc_conf_exists() {
        print!("{}", rs.read_rc_conf());
    }and the first half of the class looks like pa
}
