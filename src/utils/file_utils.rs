use encoding_rs::UTF_8;
use std::fs::{self, File};
use std::io::{Error, Read};
use std::path::Path;

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
pub fn get_first_html_file(path: &Path) -> Result<String, Error> {
    let paths = fs::read_dir(path)?;
    let mut file_name = String::new();
    for path in paths {
        let path = path?.path();
        if path.is_file() {
            let path_string = path.to_string_lossy();
            let extension = path_string.split('.').last().unwrap_or_default();
            if extension == "html" {
                file_name = path_string.to_string();
                break;
            }
        }
    }
    Ok(file_name)
}

// Reads the contents of a file and returns the contents.
fn get_file_contents(path: &Path) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let (encoding, _) = UTF_8.decode_with_bom_removal(&contents);
    let contents = encoding.to_string();

    Ok(contents)
}
