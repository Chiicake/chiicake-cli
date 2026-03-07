use clap::{ArgAction, Parser};
use rand::{rng, RngExt};
use zxcvbn::zxcvbn;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(
        short,
        long,
        help = "Service name for the password",
        default_value = "default"
    )]
    pub service: String,

    #[arg(short, long, help = "Password length", default_value_t = 12)]
    pub length: usize,

    #[arg(
        long = "no-uc",
        help = "Do not include uppercase letters in the password",
        default_value_t = true,
        action = ArgAction::SetFalse
    )]
    pub uc: bool,

    #[arg(
        long = "no-lc",
        help = "Do not include lowercase letters in the password",
        default_value_t = true,
        action = ArgAction::SetFalse
    )]
    pub lc: bool,

    #[arg(
        long = "no-dg",
        help = "Do not include digits in the password",
        default_value_t = true,
        action = ArgAction::SetFalse
    )]
    pub dg: bool,

    #[arg(
        long = "no-sp",
        help = "Do not include special characters in the password",
        default_value_t = true,
        action = ArgAction::SetFalse
    )]
    pub sp: bool,
}

pub fn generate_pass(opts: &GenPassOpts) -> Result<(), Box<dyn std::error::Error>> {
    let mut charset = String::new();
    if opts.uc {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if opts.lc {
        charset.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if opts.dg {
        charset.push_str("0123456789");
    }
    if opts.sp {
        charset.push_str("!@#$%^&*?");
    }
    if charset.is_empty() {
        return Err("At least one character type must be included".into());
    }

    let mut password = String::new();
    let mut rng = rng();
    for _ in 0..opts.length {
        let idx = rng.random::<i8>() as usize % charset.len();
        password.push(charset.chars().nth(idx).unwrap());
    }

    println!("{}", password);
    println!("Password strength: {}", zxcvbn(&password, &[]).score());
    Ok(())
}
