pub mod entry;
mod file;
mod html;

use clap::Parser;
use std::{path::Path, process::exit};

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

    /// Inject some html to <head> of the index html
    #[arg(long, value_name = "HTML")]
    inject: Option<String>,

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

    // --dir
    let dirpath = Path::new(&args.dir);

    // --all
    let entry = if args.all {
        entry::Entry::new_all(dirpath)
    } else {
        entry::Entry::new(dirpath)
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

            // --name
            let index_file_name = args.name;

            if args.remove {
                // --remove
                let removed_count = rm_index(&entry, &dirpath.to_path_buf(), &index_file_name);
                print!(
                    "\nRemoved {}",
                    auto_s(removed_count, "index file", "index files"),
                );
                return;
            }

            // --inject
            let inject = match args.inject {
                Some(code) => code,
                None => "".to_string(),
            };

            let total_count = entry.count_dir();

            // --force
            let success_count = file::gen_index(
                &entry,
                &dirpath.to_path_buf(),
                &index_file_name,
                args.force,
                &inject,
            );

            print!(
                "\nGenerated {} for {}",
                auto_s(success_count, "index file", "index files"),
                auto_s(total_count, "directory", "directories")
            );
        }
        Err(e) => {
            eprintln!("--dir={}\n{}", args.dir, e);
            exit(1)
        }
    }
}
