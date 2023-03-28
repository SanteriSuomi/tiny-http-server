mod communication;
mod utils;

use crate::communication::request::handle_request;
use crate::communication::response::handle_response;

use std::error::Error;
use std::net::TcpListener;

pub fn run() -> Result<(), Box<dyn Error>> {
    let listener = match TcpListener::bind("127.0.0.1:5000") {
        Ok(listener) => listener,
        Err(e) => return Err(Box::new(e)),
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let request = match handle_request(&stream) {
                    Ok(request) => request,
                    Err(e) => return Err(e),
                };
                if let Err(e) = handle_response(&stream, &request) {
                    return Err(Box::new(e));
                }
            }
            Err(e) => return Err(Box::new(e)),
        }
    }

    Ok(())
}
