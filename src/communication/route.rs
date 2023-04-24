use std::{collections::HashMap, sync::Arc};

use super::{method::Method, request::Request, response::Response};

// User defined function type found at every defined route (path)
pub type RouteFunc = Arc<dyn Fn(&Request, &mut Response) -> () + Send + Sync + 'static>;

#[derive(Clone)]
pub struct Route {
    pub method_map: HashMap<Method, RouteFunc>,
}

impl Route {
    pub fn new(method_map: Option<HashMap<Method, RouteFunc>>) -> Self {
        match method_map {
            Some(m) => Self { method_map: m },
            None => Self {
                method_map: HashMap::new(),
            },
        }
    }
}
