use tiny_rust_server::Server;

fn main() {
    match Server::new("127.0.0.1", "5000", 5) {
        Ok(mut server) => {
            server.register_route("/test", "GET", |request, response| {
                println!("GET Request: {:#?}", request);
            });
            server.register_route("/test", "POST", |request, response| {
                println!("POST Request: {:#?}", request);
            });
            match server.run() {
                Ok(_) => println!("Server running..."),
                Err(e) => println!("Server Error: {:#?}", e),
            }
        }
        Err(e) => println!("Error: {:#?}", e),
    }
}
