pub mod base64;
pub mod csv;
pub mod pass_mgmt;

fn verify_input_file(filename: &str) -> Result<String, String> {
    if !std::path::Path::new(filename).exists() {
        return Err(format!("Input file '{}' does not exist", filename));
    }
    Ok(filename.to_string())
}
