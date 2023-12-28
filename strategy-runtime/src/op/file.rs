use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

#[inline]
pub fn is_exist(path: &str) -> bool {
    fs::metadata(path).is_err()
}

#[inline]
pub fn read(path: &str) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e.to_string()),
    }
}

#[inline]
pub fn write(path: &str, contents: &str) -> Result<(), String> {
    fs::write(path, contents).map_err(|e| e.to_string())
}

#[inline]
pub fn append(path: &str, contents: &str) -> Result<(), String> {
    OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .and_then(|mut it| write!(it, "{}", contents))
        .map_err(|e| e.to_string())
}
