use std::error::Error;
use std::io::{self, ErrorKind};

// Representation of the supported HTTP methods.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Method {
    GET(String),
    POST(String),
    PUT(String),
    DELETE(String),
}

impl Method {
    pub fn default() -> Self {
        Method::GET(String::from("GET"))
    }

    pub fn from_str(method: &str) -> Result<Method, Box<dyn Error>> {
        match method {
            "GET" => Ok(Method::GET(String::from("GET"))),
            "POST" => Ok(Method::POST(String::from("POST"))),
            "PUT" => Ok(Method::PUT(String::from("PUT"))),
            "DELETE" => Ok(Method::DELETE(String::from("DELETE"))),
            _ => Err(Box::new(io::Error::new(
                ErrorKind::Other,
                "Non-supported request method",
            ))),
        }
    }

    pub fn get_str_vec() -> Vec<&'static str> {
        vec!["GET", "POST", "PUT", "DELETE"]
    }
}
