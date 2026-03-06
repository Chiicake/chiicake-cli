use crate::csv_process::{execute_csv, CsvOpts};
use clap::Parser;
use std::error::Error;

#[derive(Debug, Parser)]
#[command(name = "chiicake-cli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to other formats")]
    Csv(CsvOpts),
}

pub fn execute_opt(opts: Opts) -> Result<(), Box<dyn Error>> {
    match opts.cmd {
        SubCommand::Csv(csv_opts) => execute_csv(&csv_opts)?,
    }
    Ok(())
}
