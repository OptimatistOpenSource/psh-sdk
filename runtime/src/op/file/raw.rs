use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

#[inline]
pub fn is_exist(path: &str) -> bool {
    Path::new(path).exists()
}

#[inline]
pub fn read(path: &str) -> io::Result<String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e),
    }
}

#[inline]
pub fn write(path: &str, contents: &str) -> io::Result<()> {
    fs::write(path, contents)
}

#[inline]
pub fn append(path: &str, contents: &str) -> io::Result<()> {
    OpenOptions::new()
        .append(true)
        .open(path)
        .and_then(|mut it| write!(it, "{}", contents))
}

#[inline]
pub fn remove_file(path: &str) -> io::Result<()> {
    fs::remove_file(path)
}

#[inline]
pub fn create_dir(path: &str) -> io::Result<()> {
    fs::create_dir_all(path)
}

#[inline]
pub fn remove_dir(path: &str) -> io::Result<()> {
    fs::remove_dir_all(path)
}
