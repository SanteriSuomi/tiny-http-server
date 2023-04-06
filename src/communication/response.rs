use std::{io::Error, io::Write, net::TcpStream, path::Path};

use crate::utils::file_utils::{get_first_html_file, read_file_with_extension};

use super::request::Request;

pub struct Response {
    pub status_code: String,
    pub status_message: String,
    pub content_type: String,
    pub content: String,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status_code: String::new(),
            status_message: String::new(),
            content_type: String::new(),
            content: String::new(),
        }
    }

    pub fn send(&mut self, mut stream: &TcpStream) -> Result<(), Error> {
        let status_code = &self.status_code;
        let status_message = &self.status_message;
        let content_type = &self.content_type;
        let content_string = &self.content;
        let content_length = content_string.len();

        let format = format!(
            "HTTP/1.1 {status_code} {status_message}\r\n\
             Content-Type: text/{content_type}\r\n\
             Content-Length: {content_length}\r\n\
             \r\n\
             {content_string}",
        );
        stream.write_all(format.as_bytes())
    }

    pub fn set_status_code(&mut self, status_code: &str) {
        self.status_code = status_code.to_string();
    }

    pub fn set_status_message(&mut self, status_message: &str) {
        self.status_message = status_message.to_string();
    }

    pub fn set_content(&mut self, content: &str, content_type: &str) {
        self.content = content.to_string();
        self.content_type = content_type.to_string();
    }

    // fn handle_response(&mut self, mut stream: &TcpStream, request: &Request) -> Result<(), Error> {
    //     // let (status_code, content_string, content_length, content_type) =
    //     //     match Self::build_response(request) {
    //     //         Ok(response) => response,
    //     //         Err(e) => return Err(e),
    //     //     };

    //     let status_code = &self.status_code;
    //     let status_message = &self.status_message;
    //     let content_type = &self.content_type;
    //     let content_string = &self.content;
    //     let content_length = content_string.len();

    //     let format = format!(
    //         "HTTP/1.1 {status_code} {status_message}\r\n\
    //          Content-Type: text/{content_type}\r\n\
    //          Content-Length: {content_length}\r\n\
    //          \r\n\
    //          {content_string}",
    //     );
    //     stream.write_all(format.as_bytes())
    // }

    // fn build_response(request: &Request) -> Result<(String, String, usize, String), Error> {
    //     let mut status_code = String::from("404 Not Found");

    //     let mut content_string = String::new();
    //     let mut content_length = 0;
    //     let mut content_type = String::new();

    //     let path = match request.path.as_str() {
    //         "/" => get_first_html_file(Path::new("public")).unwrap_or_default(),
    //         _ => format!("public{}", request.path),
    //     };
    //     let path = Path::new(path.as_str());
    //     if path.exists() {
    //         content_string = match read_file_with_extension(path) {
    //             Ok((contents, extension)) => {
    //                 status_code = String::from("200 OK");
    //                 content_length = contents.len();
    //                 content_type = extension;
    //                 contents
    //             }
    //             Err(e) => return Err(e),
    //         };
    //     }

    //     Ok((status_code, content_string, content_length, content_type))
    // }
}
