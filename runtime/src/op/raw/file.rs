use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[inline]
pub fn is_exist(path: &str) -> bool {
    Path::new(path).exists()
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
        .append(true)
        .open(path)
        .and_then(|mut it| write!(it, "{}", contents))
        .map_err(|e| e.to_string())
}

#[inline]
pub fn remove_file(path: &str) -> Result<(), String> {
    fs::remove_file(path).map_err(|e| e.to_string())
}

#[inline]
pub fn create_dir(path: &str) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|e| e.to_string())
}

#[inline]
pub fn remove_dir(path: &str) -> Result<(), String> {
    fs::remove_dir_all(path).map_err(|e| e.to_string())
}
