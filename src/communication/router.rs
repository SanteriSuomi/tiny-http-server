use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{ds::trie::Trie, log};

use super::{
    method::Method,
    middleware::Middleware,
    request::Request,
    response::Response,
    route::{Route, RouteFunc},
};

#[derive(Clone)]
pub struct Router {
    pub base_path: String,
    middleware: Vec<Middleware>,
    routes: Arc<Mutex<Trie<Route>>>,
}

impl Router {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: String::from(base_path),
            middleware: Vec::new(),
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

    // Register a middleware with the router.
    pub fn middleware<F>(&mut self, func: F)
    where
        F: Fn(Option<&Middleware>, &mut Request) -> () + Send + Sync + 'static,
    {
        self.middleware.push(Middleware::new(func));
    }

    // Find a route for the router given a request. Returns None if no route is found.
    pub fn find_route(&self, request: &mut Request) -> Option<Route> {
        self.routes.lock().unwrap().search(&request.path)
    }

    pub fn execute_middleware(&self, request: &mut Request) {
        let mut prev_mid: Option<&Middleware> = None;
        if self.middleware.len() > 0 {
            for mid in &self.middleware {
                (mid.func)(prev_mid, request);
                prev_mid = Some(mid);
            }
        }
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
                    Some(route) => {
                        let mut method_map = route.method_map;
                        assert!(
                            !method_map.contains_key(&method),
                            "Route already exists for path: {} and method: {:#?}",
                            path,
                            method
                        );
                        Self::insert_route(&mut method_map, method, func);
                    }
                    None => {
                        let mut method_map = HashMap::new();
                        Self::insert_route(&mut method_map, method, func);
                        routes.insert(path.as_str(), Route::new(Some(method_map)));
                    }
                }
            }
            Err(e) => log!("Method Error: {:#?}", e),
        }
    }

    fn insert_route<F>(method_map: &mut HashMap<Method, RouteFunc>, method: Method, func: F)
    where
        F: Fn(&Request, &mut Response) -> () + Send + Sync + 'static,
    {
        method_map.insert(method, Arc::new(func));
    }
}
