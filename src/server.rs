use crate::communication::method::Method;
use crate::communication::request::{Request, StaticRequestData};

use crate::communication::response::Response;
use crate::utils::file_utils::get_first_html_file_name;
use crate::utils::general_utils::is_static_file;
use crate::utils::guess_utils::guess_mime_type;
use crate::utils::thread_pool::ThreadPool;

use std::collections::HashMap;
use std::env::current_dir;
use std::error::Error;
use std::fmt;
use std::fs::read_to_string;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};

// This is the main entry point for the server.
pub struct Server {
    thread_pool: ThreadPool,
    listener: TcpListener,
    route_map: Arc<Mutex<HashMap<String, HashMap<Method, Route>>>>,
    _address: String,
    root_path: String,
}

#[derive(Clone)]
struct Route {
    func: Arc<dyn Fn(&Request, &mut Response) -> () + Send + Sync + 'static>,
}

struct Address {
    ip: (usize, usize, usize, usize),
    port: usize,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{ip0}.{ip1}.{ip2}.{ip3}:{port}",
            ip0 = self.ip.0,
            ip1 = self.ip.1,
            ip2 = self.ip.2,
            ip3 = self.ip.3,
            port = self.port
        )
    }
}

impl Server {
    pub fn new(ip: (usize, usize, usize, usize), port: usize) -> Result<Server, Box<dyn Error>> {
        let _address: String = Address { ip, port }.to_string();
        match TcpListener::bind(&_address) {
            Ok(listener) => {
                return Ok(Server {
                    thread_pool: ThreadPool::new(5),
                    listener,
                    route_map: Arc::new(Mutex::new(HashMap::new())),
                    _address,
                    root_path: current_dir().unwrap_or_default().display().to_string(),
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
                            Ok(mut request) => {
                                Self::handle_loop(&map, &stream, &mut request);
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

    // Execute main request-response "loop" logic for the server.
    fn handle_loop(
        route_map: &Arc<Mutex<HashMap<String, HashMap<Method, Route>>>>,
        stream: &TcpStream,
        request: &mut Request,
    ) {
        // Check if the request is for a static file.
        Self::check_static_request(request);
        let mut response = Response::new();
        Self::match_route(route_map, request, &mut response);
        println!("Response: {:#?}", response);
        if let Err(e) = response.send(stream) {
            println!("Response Error: {:#?}", e);
        }
    }

    // Check if the request is for a static file, and add the static request data to the request object if so. Also change to forward to the static route.
    fn check_static_request(request: &mut Request) {
        if request.path == "/" {
            request.static_request_data = Some(StaticRequestData { path: None });
            request.path = String::from("/static");
        } else {
            let static_info = Self::is_static_path(request);
            if static_info.0 {
                request.static_request_data = Some(StaticRequestData {
                    path: Some(static_info.1),
                });
                request.path = String::from("/static");
            }
        }
    }

    // Static method to check if the request path is a static file.
    fn is_static_path(request: &Request) -> (bool, String) {
        let mut is_static = false;
        let path = Path::new(&request.path);
        match path.extension() {
            Some(extension) => {
                is_static = is_static_file(&extension.to_string_lossy());
            }
            None => {}
        }
        (is_static, path.to_string_lossy().to_string())
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
        assert!(path != "/static", "Cannot register route for static path.");
        self.create_route(path, method, func);
    }

    fn create_route<F>(&mut self, path: &str, method: &str, func: F)
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

    // Register a route with the server that serves static files from a directory starting from project root.
    pub fn serve_static(&mut self, dir: &str) {
        let root_path = format!("{}\\{}", self.root_path, dir);
        self.create_route("/static", "GET", move |request, response| {
            if let Some((path, extension)) = Self::get_static_file_details(request, &root_path) {
                match read_to_string(path) {
                    Ok(file_content) => {
                        response.set_contents(&guess_mime_type(&extension), &file_content)
                    }
                    Err(e) => {
                        response.set_status(404, "Not Found");
                        println!("File Read Error: {:#?}", e);
                    }
                }
            }
        });
    }

    // Static method to get the file details (path, extension) for a static file request.
    fn get_static_file_details(request: &Request, root_path: &str) -> Option<(String, String)> {
        if let Some(ref data) = request.static_request_data {
            if let Some(ref path) = data.path {
                return Some((
                    format!("{}\\{}", root_path, path),
                    path.split('.').last().unwrap_or("text/plain").to_string(),
                ));
            } else {
                let (resource, extension) = match get_first_html_file_name(Path::new(&root_path)) {
                    Ok(file) => file,
                    Err(e) => {
                        println!("Get File Details Error: {:#?}", e);
                        (
                            request.path.clone(),
                            request
                                .path
                                .split('.')
                                .last()
                                .unwrap_or("text/plain")
                                .to_string(),
                        )
                    }
                };
                return Some((format!("{}\\{}", root_path, resource), extension));
            }
        }
        None
    }
}