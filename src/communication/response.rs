use crate::utils::guess_utils::guess_mime_type;
use std::{io::Error, io::Write, net::TcpStream};

#[derive(Debug)]
pub struct Response {
    pub status_code: usize,
    pub status_message: String,
    pub content_type: Option<String>,
    pub content: Option<String>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status_code: 200,
            status_message: String::from("OK"),
            content_type: None,
            content: None,
        }
    }

    // Send this response back to the client.
    pub fn send(&mut self, stream: &TcpStream) -> Result<(), Error> {
        match (&self.content_type, &self.content) {
            // If the content type is set, but the content is not, send the content type.
            (Some(content_type), Some(_)) => {
                self.send_with_content(&stream, content_type)?;
                return Ok(());
            }
            // If the content type is not set, but the content is, guess the content type and send it.
            (None, Some(content)) => {
                let content_type = match &self.content_type {
                    Some(content_type) => content_type.clone(),
                    None => guess_mime_type(content.as_str()),
                };
                self.send_with_content(&stream, &content_type)?;
                return Ok(());
            }
            // If neither the content type nor the content is set, send no content.
            _ => {
                self.send_without_content(&stream)?;
                return Ok(());
            }
        }
    }

    fn send_with_content(&self, mut stream: &TcpStream, content_type: &str) -> Result<(), Error> {
        let status_code = &self.status_code;
        let status_message = &self.status_message;
        let content = &self.content.as_ref().unwrap();
        let content_length = content.len();

        let format = format!(
            "HTTP/1.1 {status_code} {status_message}\r\n\
             Content-Type: {content_type}\r\n\
             Content-Length: {content_length}\r\n\
             \r\n\
             {content}",
        );
        stream.write_all(format.as_bytes())
    }

    fn send_without_content(&self, mut stream: &TcpStream) -> Result<(), Error> {
        let status_code = &self.status_code;
        let status_message = &self.status_message;
        let format = format!("HTTP/1.1 {status_code} {status_message}\r\n\"",);
        stream.write_all(format.as_bytes())
    }

    pub fn set_status(&mut self, status_code: usize, status_message: &str) {
        self.status_code = status_code;
        self.status_message = status_message.to_string();
    }

    pub fn set_status_code(&mut self, status_code: usize) {
        self.status_code = status_code;
    }

    pub fn set_status_message(&mut self, status_message: &str) {
        self.status_message = status_message.to_string();
    }

    pub fn set_contents(&mut self, content_type: &str, content: &str) {
        self.content_type = Some(content_type.to_string());
        self.content = Some(content.to_string());
    }

    pub fn set_content_type(&mut self, content_type: &str) {
        self.content_type = Some(content_type.to_string());
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = Some(content.to_string());
    }
}
