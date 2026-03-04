fn main() {
    println!("Hello, world! I'm Chiicake");
}

// test
#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert_eq!(2 + 2, 4);
    }
}
