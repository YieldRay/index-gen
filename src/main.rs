pub mod entry;
mod file;
mod html;

use clap::Parser;
use std::{path::Path, process::exit};

/// Generate index.html file recursively for a directory
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Root dir to generate, default is current dir
    #[arg(short, long, value_name = "DIR", default_value_t = String::from("."))]
    dir: String,

    /// The index file name
    #[arg(short, long, value_name = "NAME", default_value_t = String::from("index.html"))]
    name: String,

    /// Do not ignore entries starting with `.`
    #[arg(short, long, default_value_t = false)]
    all: bool,

    /// Override if the index file already exists
    #[arg(short, long, default_value_t = false)]
    force: bool,

    /// Do not generate file, only print JSON
    #[arg(long, default_value_t = false)]
    json: bool,

    /// Do not generate file, only print String
    #[arg(long, default_value_t = false)]
    string: bool,
}

fn main() {
    let args = Args::parse();
    let root_dir = args.dir;
    let force = args.force;

    let path = Path::new(&root_dir);

    let entry = if args.all {
        entry::Entry::new_all(path)
    } else {
        entry::Entry::new(path)
    };

    match entry {
        Ok(entry) => {
            if args.json && args.string {
                eprintln!("Cannot print both json and string");
                exit(1);
            }

            if args.json {
                entry.print_json();
                return;
            }

            if args.string {
                entry.print();
                return;
            }

            // start to gen index
            let index_file_name = args.name;

            let total_count = entry.count_dir();

            let success_count = file::gen_index(&entry, ".", &index_file_name, force);

            let auto_s = |c: usize, s: &str, ss: &str| {
                if c > 0 {
                    format!("{} {}", c, ss)
                } else {
                    format!("{} {}", c, s)
                }
            };

            print!(
                "\nGenerated {} for {}",
                auto_s(success_count, "index file", "index files"),
                auto_s(total_count, "directory", "directories")
            );
        }
        Err(e) => {
            eprintln!("--dir={}\n{}", root_dir, e);
            exit(1)
        }
    }
}
