// ccli csv -i input.csv -o output.json --header -d ',s'

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "ccli", version, about, long_about = None)]
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
    #[arg(short, long, help = "Input CSV file")]
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

fn main() {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            println!("Input: {}", csv_opts.input);
            println!("Output: {}", csv_opts.output);
            println!("Header: {}", csv_opts.header);
            println!("Delimiter: '{}'", csv_opts.delimiter);
            // Here you would add the logic to read the CSV file, process it, and write the output
        }
    }
}

// test
#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert_eq!(2 + 2, 4);
    }
}
