use clap::Parser;
use rand::random;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    /// 密码对应的服务名称
    #[arg(short, long, help = "Service name for the password")]
    pub service: String,

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
    #[arg(long, help = "Include digits in the password", default_value_t = true)]
    pub dg: bool,

    /// Include special characters in the password
    #[arg(
        long,
        help = "Include special characters in the password",
        default_value_t = true
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
        charset.push_str("!@#$%^&*()-_=+[]{}|;:,.<>?/");
    }

    let mut password = String::new();
    for _ in 0..opts.length {
        let idx = random::<i8>() as usize % charset.len();
        password.push(charset.chars().nth(idx).unwrap());
    }

    println!("{}", password);
    Ok(())
}
