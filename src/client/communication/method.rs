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

    pub fn from_str(method: &str) -> Option<Method> {
        match method {
            "GET" => Some(Method::GET(String::from("GET"))),
            "POST" => Some(Method::POST(String::from("POST"))),
            "PUT" => Some(Method::PUT(String::from("PUT"))),
            "DELETE" => Some(Method::DELETE(String::from("DELETE"))),
            _ => None,
        }
    }

    pub fn get_str_vec() -> Vec<&'static str> {
        vec!["GET", "POST", "PUT", "DELETE"]
    }
}
