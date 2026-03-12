use crate::process::base64::{execute_base64, Base64Subcommand};
use crate::process::csv::{execute_csv, CsvOpts};
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
    #[command(subcommand)]
    Base64(Base64Subcommand),
}

pub fn execute_opt(opts: Opts) -> Result<(), Box<dyn Error>> {
    match opts.cmd {
        SubCommand::Csv(csv_opts) => execute_csv(&csv_opts)?,
        SubCommand::GenPass(genpass_opts) => generate_pass(&genpass_opts)?,
        SubCommand::Base64(base64_subcommand) => execute_base64(base64_subcommand)?,
    }
    Ok(())
}
