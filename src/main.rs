use bf_rust;
use clap::Parser;
use std::path;

/// Simple program to get the top biggest files in a directory
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the directory to search for the biggest files
    #[arg(short, long)]
    path: String,

    /// Number of files to return
    #[arg(short, long, default_value_t = 10)]
    count: usize,
}

fn main() {
    let args = Args::parse();

    bf_rust::show(bf_rust::biggest_files_in_dir(
        path::Path::new(&args.path).to_path_buf(),
        args.count,
    ))
}
