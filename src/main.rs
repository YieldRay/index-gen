#![recursion_limit = "256"]
pub mod entry;
pub mod html;

use clap::Parser;
use std::path::Path;

/// Generate index.html (or other name) for a diretory
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

    /// Print JSON
    #[arg(short, long, default_value_t = false)]
    json: bool,
}

fn main() {
    let args = Args::parse();

    let root_dir = args.dir;
    let entry = entry::Entry::new(&Path::new(&root_dir)).unwrap();

    if args.json {
        entry.print_json();
        return;
    }

    let index_file_name = args.name;
    // html::gen_index(&entry, &index_file_name);

    println!("{}", html::html_for_dir(&entry, ".", &index_file_name));
}
