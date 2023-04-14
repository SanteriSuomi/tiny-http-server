use std::fs;
use std::io::Error;
use std::path::Path;

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
