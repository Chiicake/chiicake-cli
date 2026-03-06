use ccli::opts;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = opts::Opts::parse();
    opts::execute_opt(opts)?;
    Ok(())
}

// test
#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert_eq!(2 + 2, 4);
    }
}
