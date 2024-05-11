use std::{path::PathBuf, path::Path, process::exit};
use clap::{Arg, Command};
use uniq::*;

#[derive(Debug)]
struct Args {
    operation: String,
    file_path: String,
}

fn get_args() -> Args {
    let  uniq = Command::new("uniq")
      .arg(
            Arg::new("operation")
              .required(true)
              .aliases(["l","-","c","d","u"])
              .help("Sets the operation to perform"),
              
        )
      .arg(
            Arg::new("file")
              .required(true)
              .help("Sets the input file to process"),
        )
      .get_matches();

    Args {
        operation: uniq.get_one::<String>("operation").cloned().unwrap(),
        file_path: uniq.get_one::<String>("file").cloned().unwrap(),
    }
}

fn main() {
    let args = get_args();

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
