use crate::client::communication::method::Method;
use crate::client::communication::request::Request;

use crate::client::communication::response::Response;
use crate::client::utils::thread_pool::ThreadPool;

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
    func: Arc<dyn Fn(&Request, &mut Response) -> () + Send + Sync + 'static>,
}

impl Server {
    pub fn new(port: usize) -> Result<Server, Box<dyn Error>> {
        let _address = format!("127.0.0.1:{port}");
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
        println!("Server starting...");
        for stream in self.listener.incoming() {
            let map = self.route_map.clone();
            match stream {
                Ok(stream) => {
                    self.thread_pool
                        .execute(move || match Request::handle_request(&stream) {
                            Ok(request) => {
                                let mut response = Response::new();
                                Self::match_route(&map, &request, &mut response);
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
        Ok(())
    }

    // Static method to match the request to the correct route, and call user registered function found on that route.
    fn match_route(
        map: &Arc<Mutex<HashMap<String, HashMap<Method, Route>>>>,
        request: &Request,
        response: &mut Response,
    ) {
        if let Some(user_request) = map
            .lock()
            .unwrap()
            .get(&request.path)
            .and_then(|routes| routes.get(&request.method))
        {
            (user_request.func)(request, response);
        }
    }

    // Register a route with the server.
    pub fn route<F>(&mut self, path: &str, method: &str, func: F)
    where
        F: Fn(&Request, &mut Response) -> () + Send + Sync + 'static,
    {
        match Method::from_str(method) {
            Some(method) => {
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
            None => println!("Invalid method: {}", method),
        }
    }
}
