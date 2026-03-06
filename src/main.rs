// ccli csv -i input.csv -o output.json --header -d ',s'

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[command(name = "chiicake-cli", version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
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

//Name,Position,DOB,Nationality,Kit Number
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            let mut reader = csv::ReaderBuilder::new().from_path(&csv_opts.input)?;
            let record = reader
                .deserialize()
                .map(|result| result.expect("Failed to deserialize CSV"))
                .collect::<Vec<Player>>();

            println!("{:?}", record);
            let json = serde_json::to_string_pretty(&record)?;
            std::fs::write(&csv_opts.output, json)?;
            println!("Wrote to {}", csv_opts.output);
        }
    }

    Ok(())
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if !std::path::Path::new(filename).exists() {
        return Err(format!("Input file '{}' does not exist", filename));
    } else if !filename.ends_with(".csv") {
        return Err(format!("Input file '{}' is not a CSV file", filename));
    }
    Ok(filename.to_string())
}

// test
#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert_eq!(2 + 2, 4);
    }
}
