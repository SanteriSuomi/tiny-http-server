use encoding_rs::UTF_8;
use std::fs::File;
use std::io::{Error, Read};

pub fn read_file_contents(path: &str) -> Result<(String, String), Error> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let (encoding, _) = UTF_8.decode_with_bom_removal(&contents);
    let contents = encoding.to_string();

    let extension = match path.split('.').last().unwrap_or_default() {
        "html" => "html",
        "css" => "css",
        "js" => "javascript",
        _ => "plain",
    }
    .to_string();

    Ok((contents, extension))
}
