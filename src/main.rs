#![recursion_limit = "256"]
mod entry;
mod file;
mod html;

use clap::Parser;
use std::path::Path;

/// Generating index.html file recursively for a directory
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

    /// Do not generate file, only print JSON
    #[arg(short, long, default_value_t = false)]
    json: bool,
}

fn main() {
    let args = Args::parse();
    let root_dir = args.dir;

    match entry::Entry::new(&Path::new(&root_dir)) {
        Ok(entry) => {
            if args.json {
                entry.print_json();
                return;
            }

            let index_file_name = args.name;
            file::gen_index(&entry, ".", &index_file_name)
        }
        Err(e) => {
            eprintln!("-dir={}\n{}", root_dir, e);
        }
    }
}
