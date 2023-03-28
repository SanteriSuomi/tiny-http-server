use std::{io::Write, net::TcpStream, path::Path};

use crate::utils::file_utils::read_file_contents;

use super::request::Request;

pub fn handle_response(mut stream: &TcpStream, request: &Request) -> Result<(), std::io::Error> {
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

fn get_response(request: &Request) -> Result<(String, String, usize, String), std::io::Error> {
    let mut status_code = String::from("404 Not Found");
    let mut content_string = String::new();
    let mut content_length = 0;
    let mut content_type = String::new();

    let path = format!("public{}", &request.path);
    if Path::new(path.as_str()).exists() {
        content_string = match read_file_contents(path.as_str()) {
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
