pub mod base64;
pub mod csv;
pub mod pass_mgmt;

fn verify_input_file(filename: &str) -> Result<String, String> {
    if filename != "-" && !std::path::Path::new(filename).exists() {
        return Err(format!("Input file '{}' does not exist", filename));
    }
    Ok(filename.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        // 创建一个临时文件用于测试
        let temp_file = std::env::temp_dir().join("test_input.txt");
        std::fs::write(&temp_file, "test content").expect("Failed to create temp file");

        // 测试存在的文件
        assert_eq!(
            verify_input_file(temp_file.to_str().unwrap()),
            Ok(temp_file.to_str().unwrap().to_string())
        );

        // 测试不存在的文件
        assert!(verify_input_file("non_existent_file.txt").is_err());

        // 清理临时文件
        std::fs::remove_file(temp_file).expect("Failed to remove temp file");
    }
}
