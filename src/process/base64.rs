use crate::process::verify_input_file;
use base64::prelude::*;
use clap::Parser;
use std::error::Error;
#[derive(Debug, Parser)]
pub enum Base64Subcommand {
    #[command(name = "encode", about = "Encode a string to Base64")]
    Encode(Base64EncodingOpts),
    #[command(name = "decode", about = "Decode a Base64 string")]
    Decode(Base64DecodingOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodingOpts {
    // default_value = "-" 表示从标准输入读取数据
    #[arg(short, long, help = "Input string to encode", value_parser = verify_input_file, default_value = "-")]
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct Base64DecodingOpts {
    #[arg(short, long, help = "Base64 string to decode", value_parser = verify_input_file, default_value = "-")]
    pub input: String,
}

pub fn execute_base64(cmd: Base64Subcommand) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        Base64Subcommand::Encode(opts) => {
            let input = read_file(opts.input.as_str())?;
            let encoded = encode_base64(&input);
            println!("{}", encoded);
        }
        Base64Subcommand::Decode(opts) => {
            let input = read_file(opts.input.as_str())?;
            let decoded = decode_base64(&input).expect("decode failed");
            println!("{}", decoded);
        }
    }
    Ok(())
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    if path == "-" {
        // 从标准输入读取数据
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_string())
    } else {
        // 从文件读取数据
        std::fs::read_to_string(path).map_err(|e| e.into())
    }
}

pub fn encode_base64(input: &str) -> String {
    BASE64_STANDARD.encode(input)
}

pub fn decode_base64(input: &str) -> Result<String, base64::DecodeError> {
    let bytes = BASE64_STANDARD.decode(input)?;
    Ok(String::from_utf8_lossy(&bytes).to_string())
}
