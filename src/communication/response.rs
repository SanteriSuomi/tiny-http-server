use std::{io::Error, io::Write, net::TcpStream, path::Path};

use crate::utils::file_utils::{get_first_html_file, read_file_with_extension};

use super::request::Request;

pub fn handle_response(mut stream: &TcpStream, request: &Request) -> Result<(), Error> {
    let (status_code, content_string, content_length, content_type) = match get_response(request) {
        Ok(response) => response,
        Err(e) => return Err(e),
    };

    let format = format!(
        "HTTP/1.1 {status_code}\r\n\
         Content-Type: text/{content_type}\r\n\
         Content-Length: {content_length}\r\n\
         \r\n\
         {content_string}",
    );
    stream.write_all(format.as_bytes())
}

fn get_response(request: &Request) -> Result<(String, String, usize, String), Error> {
    let mut status_code = String::from("404 Not Found");
    let mut content_string = String::new();
    let mut content_length = 0;
    let mut content_type = String::new();

    let path = match request.path.as_str() {
        "/" => get_first_html_file(Path::new("public")).unwrap_or_default(),
        _ => format!("public{}", request.path),
    };
    let path = Path::new(path.as_str());
    if path.exists() {
        content_string = match read_file_with_extension(path) {
            Ok((contents, extension)) => {
                status_code = String::from("200 OK");
                content_length = contents.len();
                content_type = extension;
                contents
            }
            Err(e) => return Err(e),
        };
    }

    Ok((status_code, content_string, content_length, content_type))
}
