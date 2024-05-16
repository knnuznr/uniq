use clap::Parser;

#[derive(Parser)]
pub(crate) struct Args {
    pub(crate) operation: String,
    pub(crate) file_path: String,
}