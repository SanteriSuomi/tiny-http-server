use crate::communication::method::Method;
use core::panic;
use std::{
    collections::HashMap,
    error::Error,
    io::{BufRead, BufReader},
    net::TcpStream,
};

// Additional data about the request used only server-side.
#[derive(Debug)]
pub struct StaticRequestData {
    pub path: Option<String>,
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub static_request_data: Option<StaticRequestData>,
}

impl Request {
    pub fn handle_request(stream: &TcpStream) -> Result<Request, Box<dyn Error>> {
        let buf_reader = BufReader::new(stream);
        let mut lines: Vec<String> = Vec::new();
        for line in buf_reader.lines() {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        break;
                    };
                    lines.push(line);
                }
                Err(e) => return Err(Box::new(e)),
            }
        }
        Ok(Self::parse_request(lines))
    }

    fn parse_request(lines: Vec<String>) -> Request {
        let mut request = Request {
            method: Method::default(),
            path: String::new(),
            headers: HashMap::new(),
            static_request_data: None,
        };

        let mut headers: Vec<String> = Vec::new();
        for line in lines {
            if Self::starts_with_http_request_type(&line) {
                let (method, path) = Self::get_request_type_info(&line);
                request.method = match Method::from_str(&method) {
                    Some(method) => method,
                    None => panic!("Non-supported request method"),
                };
                request.path = path;
            } else {
                headers.push(line);
            }
            for header in &headers {
                let split = header.split(": ").collect::<Vec<&str>>();
                if let [name, value] = split.as_slice() {
                    request.headers.insert(name.to_string(), value.to_string());
                }
            }
        }

        request
    }

    fn starts_with_http_request_type(line: &str) -> bool {
        for http_request_type in Method::get_str_vec() {
            if line.starts_with(http_request_type) {
                return true;
            }
        }
        false
    }

    fn get_request_type_info(line: &String) -> (String, String) {
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap_or_default().to_string(),
            iter.next().unwrap_or_default().to_string(),
        )
    }
}
