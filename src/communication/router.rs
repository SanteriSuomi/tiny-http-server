use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{ds::trie::Trie, log};

use super::{method::Method, request::Request, response::Response, route::Route};

#[derive(Clone)]
pub struct Router {
    pub base_path: String,
    routes: Arc<Mutex<Trie<HashMap<Method, Route>>>>,
}

impl Router {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: String::from(base_path),
            routes: Arc::new(Mutex::new(Trie::new())),
        }
    }

    // Create a route for the router.
    pub fn route<F>(&mut self, path: &str, method: &str, func: F)
    where
        F: Fn(&Request, &mut Response) -> () + Send + Sync + 'static,
    {
        self.create_route(path, method, func);
    }

    // Register a route with the router.
    fn create_route<F>(&mut self, path: &str, method: &str, func: F)
    where
        F: Fn(&Request, &mut Response) -> () + Send + Sync + 'static,
    {
        match Method::from_str(method) {
            Ok(method) => {
                let mut routes = self.routes.lock().unwrap();
                let path = format!("{}{}", self.base_path, path);
                match routes.search(path.as_str()) {
                    Some(mut method_map) => {
                        assert!(
                            !method_map.contains_key(&method),
                            "Route already exists for path: {} and method: {:#?}",
                            path,
                            method
                        );
                        method_map.insert(
                            method,
                            Route {
                                func: Arc::new(func),
                            },
                        );
                    }
                    None => {
                        let mut method_map = HashMap::new();
                        method_map.insert(
                            method,
                            Route {
                                func: Arc::new(func),
                            },
                        );
                        routes.insert(path.as_str(), method_map);
                    }
                }
            }
            Err(e) => log!("Method Error: {:#?}", e),
        }
    }

    pub fn get_route(&self, request: &Request) -> Option<Route> {
        self.routes
            .lock()
            .unwrap()
            .search(&request.path)
            .and_then(|method_map| method_map.get(&request.method).cloned())
    }
}
