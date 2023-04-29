mod file;
mod index;
mod select;

use clap::Parser;
use file::gen_index;
use select::select;
use std::{path::Path, process::exit};

use crate::file::rm_index;

/// Generate index.html file recursively for a directory
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Root dir to generate, default is current dir
    #[arg(short, long, value_name = "DIR", default_value_t = String::from("."))]
    dir: String,

    /// The index file PREFIX name, will automatically use correct extension name
    #[arg(short, long, value_name = "NAME", default_value_t = String::from("index"))]
    name: String,

    /// Override if the index file already exists
    #[arg(short, long, default_value_t = false)]
    force: bool,

    /// Inject some html to <head> of the index html
    #[arg(long, value_name = "HTML")]
    inject: Option<String>,

    /// Do not ignore entries starting with `.`
    #[arg(short, long, default_value_t = false)]
    all: bool,

    /// Recursively remove all index file
    #[arg(long, default_value_t = false)]
    remove: bool,

    /// Generate html
    #[arg(group = "output", long, default_value_t = false)]
    html: bool,

    /// Generate JSON
    #[arg(group = "output", long, default_value_t = false)]
    json: bool,

    /// Generate Txt
    #[arg(group = "output", long, default_value_t = false)]
    txt: bool,
}

fn main() {
    let args = Args::parse();

    // --dir
    let path = Path::new(&args.dir);
    if !path.is_dir() {
        eprintln!("--dir is not a directory!");
        exit(1);
    }

    let (interface, index_file_name) = select(
        // --name --all --html --json --txt --remove
        &args.name,
        args.all,
        &args.inject.unwrap_or(String::new()),
        args.html,
        args.json,
        args.txt,
    );

    let auto_s = |c: usize, s: &str, ss: &str| {
        if c > 0 {
            format!("{} {}", c, ss)
        } else {
            format!("{} {}", c, s)
        }
    };

    let done;
    let total;
    let msg;

    // --remove
    if args.remove {
        (done, total) = rm_index(path, &index_file_name);
        msg = "Removed";
    } else {
        (done, total) = gen_index(path, interface, &index_file_name, args.force);
        msg = "Generated";
    }

    println!(
        "{} {} of {}",
        msg,
        auto_s(done, "file", "files"),
        auto_s(total, "file", "files")
    );
}
