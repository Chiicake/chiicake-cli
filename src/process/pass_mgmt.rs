use clap::{ArgAction, Parser};
use rand::{rng, RngExt};
use std::io::Write;
use std::process::{Command, Stdio};
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
    if let Err(err) = copy_to_clipboard(&password) {
        eprintln!("Failed to copy password to clipboard: {err}");
    }
    eprintln!("Password strength: {}", zxcvbn(&password, &[]).score());
    Ok(())
}

fn run_clipboard_command(
    command: &str,
    args: &[&str],
    text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(text.as_bytes())?;
    }

    let status = child.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("clipboard command `{command}` failed").into())
    }
}

fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        return run_clipboard_command("pbcopy", &[], text);
    }

    #[cfg(target_os = "windows")]
    {
        return run_clipboard_command("clip", &[], text);
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        run_clipboard_command("wl-copy", &[], text)
            .or_else(|_| run_clipboard_command("xclip", &["-selection", "clipboard"], text))
            .or_else(|_| run_clipboard_command("xsel", &["--clipboard", "--input"], text))
            .map_err(|_| "no clipboard utility found (tried wl-copy/xclip/xsel)".into())
    }
}
