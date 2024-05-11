use std::{path::PathBuf, path::Path, process::exit};
use clap::Parser;
use uniq::*;

#[derive(Parser)]
struct Args {
    operation: String,
    file_path: String,
}

fn main() {
    let args = Args::parse();

    let path_buf = PathBuf::from(&args.file_path);

    let un = UNIQ::new(path_buf);

    match args.operation.as_str() {
        "l" => un.count_unique_lines(),
        "c" => un.print_counters(),
        "d" => un.print_duplicates(),
        "u" => un.print_uniq(),
        "-" => un.read_and_write(Path::new(&args.file_path)),
        _ => exit(1),
    }
}