use core::panic;
use std::{
    error::Error,
    io::{BufRead, BufReader},
    net::TcpStream,
    str::SplitWhitespace,
};

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: String,
    pub host: String,
    pub headers: Vec<String>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
}

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
    Ok(parse_request(lines))
}

fn parse_request(lines: Vec<String>) -> Request {
    let mut request = Request {
        method: Method::GET,
        path: String::new(),
        version: String::new(),
        host: String::new(),
        headers: Vec::new(),
    };

    for line in lines {
        if line.starts_with("GET") || line.starts_with("POST") {
            let (method, path, version) = get_request_info(&mut line.split_whitespace());
            request.method = match method.as_str() {
                "GET" => Method::GET,
                "POST" => Method::POST,
                _ => panic!("Invalid method."),
            };
            request.path = path;
            request.version = version
        } else if line.starts_with("Host") {
            let (_, host, _) = get_request_info(&mut line.split_whitespace());
            request.host = host;
        } else {
            request.headers.push(line);
        }
    }

    request
}

fn get_request_info(iter: &mut SplitWhitespace) -> (String, String, String) {
    (
        iter.next().unwrap_or_default().to_string(),
        iter.next().unwrap_or_default().to_string(),
        iter.next().unwrap_or_default().to_string(),
    )
}
