use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64Subcommand {
    #[command(name = "encode", about = "Encode a string to Base64")]
    Encode(Base64EncodingOpts),
    #[command(name = "decode", about = "Decode a Base64 string")]
    Decode(Base64DecodingOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodingOpts {
    #[arg(short, long, help = "Input string to encode", default_value = "")]
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct Base64DecodingOpts {
    #[arg(short, long, help = "Base64 string to decode", default_value = "")]
    pub input: String,
}

pub fn execute_base64(cmd: Base64Subcommand) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        Base64Subcommand::Encode(opts) => {
            println!("encode: {:?}", opts);
        }
        Base64Subcommand::Decode(opts) => {
            println!("decode: {:?}", opts);
        }
    }
    Ok(())
}
