use clap::Parser;
use serde::{Deserialize, Serialize};
use std::error::Error;
#[derive(Debug, Deserialize, Serialize)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u8,
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    // value parser 用于作为输入检查
    #[arg(short, long, help = "Input CSV file", value_parser = verify_input_file)]
    input: String,

    #[arg(short, long, help = "Output file", default_value = "output.json")]
    output: String,

    #[arg(long, help = "Indicates that the CSV has a header row")]
    header: bool,

    #[arg(
        short,
        long,
        help = "Delimiter used in the CSV file",
        default_value = ","
    )]
    delimiter: char,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if !std::path::Path::new(filename).exists() {
        return Err(format!("Input file '{}' does not exist", filename));
    } else if !filename.ends_with(".csv") {
        return Err(format!("Input file '{}' is not a CSV file", filename));
    }
    Ok(filename.to_string())
}

pub fn execute_csv(csv_opts: &CsvOpts) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new().from_path(&csv_opts.input)?;
    let record = reader
        .deserialize()
        .map(|result| result.expect("Failed to deserialize CSV"))
        .collect::<Vec<Player>>();

    println!("{:?}", record);
    let json = serde_json::to_string_pretty(&record)?;
    std::fs::write(&csv_opts.output, json)?;
    println!("Wrote to {}", csv_opts.output);
    Ok(())
}
