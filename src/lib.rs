mod communication;
mod utils;

use crate::communication::request::handle_request;
use crate::communication::response::handle_response;
use crate::utils::thread_pool::ThreadPool;

use std::error::Error;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

pub fn run() -> Result<(), Box<dyn Error>> {
    let listener = match TcpListener::bind("127.0.0.1:5000") {
        Ok(listener) => listener,
        Err(e) => return Err(Box::new(e)),
    };

    let thread_pool = Arc::new(Mutex::new(ThreadPool::new(5)));

    for stream in listener.incoming() {
        thread_pool.clone().lock().unwrap().execute(|| {
            if let Ok(stream) = stream {
                if let Ok(request) = handle_request(&stream) {
                    handle_response(&stream, &request);
                }
            }
        });
    }
    Ok(())
}
