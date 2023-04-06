mod communication;
mod utils;

use communication::request::{Method, Request};

use crate::communication::request::handle_request;
use crate::communication::response::Response;
use crate::utils::thread_pool::ThreadPool;

use std::collections::HashMap;
use std::error::Error;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

// This is the main entry point for the server.
pub struct Server {
    thread_pool: ThreadPool,
    listener: TcpListener,
    route_map: Arc<Mutex<HashMap<String, HashMap<Method, Route>>>>,
    _address: String,
}

#[derive(Clone)]
struct Route {
    func: Arc<dyn Fn(&Request, &Response) -> () + Send + Sync + 'static>,
}

impl Server {
    pub fn new(
        address: &str,
        port: &str,
        thread_pool_size: usize,
    ) -> Result<Server, Box<dyn Error>> {
        assert!(
            thread_pool_size > 0,
            "Thread pool size must be greater than 0."
        );
        let _address = format!("{address}:{port}");
        match TcpListener::bind(&_address) {
            Ok(listener) => {
                return Ok(Server {
                    thread_pool: ThreadPool::new(5),
                    listener,
                    route_map: Arc::new(Mutex::new(HashMap::new())),
                    _address,
                })
            }
            Err(e) => {
                println!("Listener Error: {:#?}", e);
                return Err(Box::new(e));
            }
        };
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        for stream in self.listener.incoming() {
            let map = self.route_map.clone();
            match stream {
                Ok(stream) => {
                    self.thread_pool
                        .execute(move || match handle_request(&stream) {
                            Ok(request) => {
                                let mut response = Response::new();
                                Self::match_route(&map, &request, &response);
                                if let Err(e) = response.send(&stream) {
                                    println!("Response Error: {:#?}", e);
                                }
                            }
                            Err(e) => println!("Request Error: {:#?}", e),
                        });
                }
                Err(e) => {
                    println!("Stream Error: {:#?}", e);
                    return Err(Box::new(e));
                }
            }
        }
        println!("Server starting...");
        Ok(())
    }

    // Static method to match the request to the correct route, and call user registered function found on that route.
    fn match_route(
        map: &Arc<Mutex<HashMap<String, HashMap<Method, Route>>>>,
        request: &Request,
        response: &Response,
    ) {
        if let Some(user_request) = map
            .lock()
            .unwrap()
            .get(&request.path)
            .and_then(|routes| routes.get(&request.method))
        {
            (user_request.func)(&request, &response);
        }
    }

    pub fn register_route<F>(&mut self, path: &str, method: &str, func: F)
    where
        F: Fn(&Request, &Response) -> () + Send + Sync + 'static,
    {
        let method = match method {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => panic!("Invalid method."),
        };
        let mut route_map = self.route_map.lock().unwrap();
        let method_map = route_map
            .entry(path.to_string())
            .or_insert_with(HashMap::new);
        method_map.insert(
            method,
            Route {
                func: Arc::new(func),
            },
        );
    }
}
