use crate::process::csv_process::{execute_csv, CsvOpts};
use crate::process::pass_mgmt::{generate_pass, GenPassOpts};
use clap::Parser;
use std::error::Error;

#[derive(Debug, Parser)]
#[command(name = "ccli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate random passwords")]
    GenPass(GenPassOpts),
}

pub fn execute_opt(opts: Opts) -> Result<(), Box<dyn Error>> {
    match opts.cmd {
        SubCommand::Csv(csv_opts) => execute_csv(&csv_opts)?,
        SubCommand::GenPass(genpass_opts) => generate_pass(&genpass_opts)?,
    }
    Ok(())
}
