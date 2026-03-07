use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    ///

    /// Length of the generated password
    #[arg(short, long, help = "Password length", default_value_t = 12)]
    pub length: usize,

    /// Include uppercase letters in the password
    #[arg(
        long,
        help = "Include uppercase letters in the password",
        default_value_t = true
    )]
    pub uc: bool,

    /// Include lowercase letters in the password
    #[arg(
        long,
        help = "Include lowercase letters in the password",
        default_value_t = true
    )]
    pub lc: bool,

    /// Include digits in the password
    #[arg(
        short,
        long,
        help = "Include digits in the password",
        default_value_t = true
    )]
    pub digits: bool,

    /// Include special characters in the password
    #[arg(
        short,
        long,
        help = "Include special characters in the password",
        default_value_t = true
    )]
    pub special: bool,
}

pub fn generate_pass(opts: &GenPassOpts) -> Result<(), Box<dyn std::error::Error>> {
    println!("Pass to {}", opts.length);
    Ok(())
}
