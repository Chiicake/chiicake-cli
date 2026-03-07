use clap::Parser;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

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
    let headers = reader.headers()?.clone();

    let mut ret = Vec::with_capacity(128);
    for record in reader.records() {
        let record = record?;
        let produced_record: HashMap<String, String> = headers
            .iter()
            .zip(record.iter())
            .map(|(h, v)| (h.to_string(), v.to_string()))
            .collect();
        ret.push(produced_record);
    }

    let ret = serde_json::to_string_pretty(&ret)?;
    println!("{}", ret);
    fs::write(&csv_opts.output, &ret)?;
    Ok(())
}
