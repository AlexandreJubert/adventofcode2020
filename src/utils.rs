use std::fs;

#[cfg(windows)]
pub const DOUBLE_LINE_ENDING: &str = "\r\n\r\n";

#[cfg(not(windows))]
pub const DOUBLE_LINE_ENDING: &str = "\n\n";

pub fn read_file_to_string(input_path: &'static str) -> String {
    fs::read_to_string(input_path).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", input_path);
        "".to_string()
    })
}
