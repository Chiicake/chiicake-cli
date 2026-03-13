use crate::process::verify_input_file;
use clap::Parser;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    // value parser 用于作为输入检查
    #[arg(short, long, help = "Input CSV file", value_parser = verify_input_file)]
    input: String,

    #[arg(short, long, help = "Output file", value_parser = verify_output_filename, default_value = "output")]
    output: String,

    #[arg(long, help = "Indicates that the CSV has a header row")]
    header: bool,

    #[arg(short, long, value_parser = parse_format, default_value = "json")]
    format: OutputFormat,

    #[arg(
        short,
        long,
        help = "Delimiter used in the CSV file",
        default_value = ","
    )]
    delimiter: char,
}

#[derive(Debug, Clone)]
enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

fn parse_format(s: &str) -> Result<OutputFormat, String> {
    s.try_into()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

impl TryFrom<&str> for OutputFormat {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => Err(format!("Unsupported output format '{}'", value)),
        }
    }
}

fn verify_output_filename(filename: &str) -> Result<String, String> {
    if filename.contains(".") {
        return Err(format!(
            "Output filename '{}' contains a dot, do not use filename to specify \
                    output format, use -f (json by default)",
            filename
        ));
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

    let out_format = csv_opts.format.clone();
    let ret_str: String = match out_format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Toml => toml::to_string(&ret)?,
    };
    println!("{}", ret_str);
    let out_format_str: &str = out_format.into();
    let out_filename = format!("{}.{}", csv_opts.output, out_format_str);
    fs::write(out_filename, &ret_str)?;
    Ok(())
}
