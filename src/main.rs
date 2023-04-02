pub mod entry;
mod file;
mod html;

use clap::Parser;
use std::{
    path::{Path, PathBuf},
    process::exit,
};

use crate::file::rm_index;

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

    /// Override if the index file already exists
    #[arg(short, long, default_value_t = false)]
    force: bool,

    /// Do not ignore entries starting with `.`
    #[arg(short, long, default_value_t = false)]
    all: bool,

    /// Recursively remove all index file
    #[arg(group = "output", long, default_value_t = false)]
    remove: bool,

    /// Do not generate file, only print JSON
    #[arg(group = "output", long, default_value_t = false)]
    json: bool,

    /// Do not generate file, only print String
    #[arg(group = "output", long, default_value_t = false)]
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
            if args.json {
                // --json
                entry.print_json();
                return;
            }

            if args.string {
                // --string
                entry.print();
                return;
            }

            // util function aims to add `s` to the end of a word
            let auto_s = |c: usize, s: &str, ss: &str| {
                if c > 0 {
                    format!("{} {}", c, ss)
                } else {
                    format!("{} {}", c, s)
                }
            };

            let index_file_name = args.name;

            if args.remove {
                // --remove
                let removed_count = rm_index(&entry, &PathBuf::from(root_dir), &index_file_name);
                print!(
                    "\nRemoved {}",
                    auto_s(removed_count, "index file", "index files"),
                );
                return;
            }

            // start to gen index
            let total_count = entry.count_dir();
            let success_count = file::gen_index(
                &entry,
                &PathBuf::from(root_dir),
                &index_file_name,
                force,
                "",
            );

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
