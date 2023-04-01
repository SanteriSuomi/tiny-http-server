mod communication;
mod utils;

use crate::communication::request::handle_request;
use crate::communication::response::handle_response;
use crate::utils::thread_pool::ThreadPool;

use std::error::Error;
use std::net::TcpListener;

pub fn run() -> Result<(), Box<dyn Error>> {
    let listener = match TcpListener::bind("127.0.0.1:5000") {
        Ok(listener) => listener,
        Err(e) => return Err(Box::new(e)),
    };

    let mut thread_pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread_pool.execute(move || match handle_request(&stream) {
                    Ok(request) => {
                        if let Err(e) = handle_response(&stream, &request) {
                            println!("Response Error: {:#?}", e);
                        }
                    }
                    Err(e) => println!("Request Error: {:#?}", e),
                });
            }
            Err(e) => println!("Stream Error: {:#?}", e),
        }
    }
    Ok(())
}
