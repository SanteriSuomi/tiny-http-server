use std::fs::{self, File};
use std::io::{Error, Read};
use std::path::Path;
use std::str::from_utf8;

/// Reads the contents of a file and returns the contents and the file extension.
pub fn read_file_with_extension(path: &Path) -> Result<(String, String), Error> {
    let extension = match path.to_string_lossy().split('.').last().unwrap_or_default() {
        "html" => "html",
        "css" => "css",
        "js" => "javascript",
        _ => "plain",
    }
    .to_string();
    Ok((get_file_contents(path)?, extension))
}

// Gets the first HTML file name in a directory.
pub fn get_first_html_file_name(path: &Path) -> Result<(String, String), Error> {
    let paths = fs::read_dir(path)?;
    for path in paths {
        let path = path?.path();
        match (path.is_file(), path.extension()) {
            (true, Some(extension)) => {
                if extension == "html" {
                    return Ok((
                        path.file_name().unwrap().to_string_lossy().to_string(),
                        path.extension().unwrap().to_string_lossy().to_string(),
                    ));
                }
            }
            _ => continue,
        }
    }
    Err(Error::new(
        std::io::ErrorKind::NotFound,
        "No HTML files found.",
    ))
}

// Reads the contents of a file and returns the contents.
fn get_file_contents(path: &Path) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(from_utf8(&contents).unwrap_or_default().to_string())
}
