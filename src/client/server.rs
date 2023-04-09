use crate::client::communication::method::Method;
use crate::client::communication::request::{Request, StaticRequestData};

use crate::client::communication::response::Response;
use crate::client::utils::file_utils::get_first_html_file_name;
use crate::client::utils::general_utils::is_static_file;
use crate::client::utils::guess_utils::guess_mime_type;
use crate::client::utils::thread_pool::ThreadPool;

use std::collections::HashMap;
use std::env::current_dir;
use std::error::Error;
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
                                Self::handle(&map, &stream, &mut request);
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

    // Execute main logic for the server.
    fn handle(
        map: &Arc<Mutex<HashMap<String, HashMap<Method, Route>>>>,
        stream: &TcpStream,
        request: &mut Request,
    ) {
        // Check if the request is for a static file.
        Self::check_static_request(request);
        let mut response = Response::new();
        Self::match_route(map, request, &mut response);
        if let Err(e) = response.send(stream) {
            println!("Response Error: {:#?}", e);
        }
    }

    // Check if the request is for a static file, and add the static request data to the request object if so.
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
            let file_path;
            let extension;
            match request.static_request_data {
                Some(ref data) => match data.path {
                    Some(ref path) => {
                        file_path = format!("{}\\{}", root_path, path);
                        extension = path.to_string();
                    }
                    None => {
                        let (resource, ext) = match get_first_html_file_name(Path::new(&root_path))
                        {
                            Ok(file) => file,
                            Err(e) => {
                                println!("File Error: {:#?}", e);
                                (request.path.clone(), String::from("text/plain"))
                            }
                        };
                        file_path = format!("{}\\{}", root_path, resource);
                        extension = ext;
                    }
                },
                None => return,
            }
            match read_to_string(file_path) {
                Ok(file) => {
                    response.set_content_type(&guess_mime_type(&extension));
                    response.set_content(&file);
                }
                Err(e) => {
                    println!("File Read Error: {:#?}", e);
                }
            }
        });
    }
}
