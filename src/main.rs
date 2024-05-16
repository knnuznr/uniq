mod cli;

use std::{path::PathBuf, process::exit};
use clap::Parser;
use uniq::*;

fn main() {
    let args = cli::Args::parse();

    let path_buf = PathBuf::from(&args.file_path);

    let un = UNIQ::new(path_buf);

    match args.operation.as_str() {
        "l" => un.count_unique_lines(),
        "c" => un.print_counters(),
        "d" => un.print_duplicates(),
        "u" => un.print_uniq(),
        "-" => un.read_and_write(),
        _ => exit(1),
    }
}