use tiny_rust_server::Server;

fn main() {
    match Server::new("127.0.0.1", "5000", 5) {
        Ok(mut server) => {
            server.register_route("/test", "GET", |request| {
                println!("GET Request: {:#?}", request);
            });
            server.register_route("/test", "POST", |request| {
                println!("POST Request: {:#?}", request);
            });
            println!("Server starting...");
            match server.run() {
                Ok(_) => println!("Server shutting down..."),
                Err(e) => println!("Error: {:#?}", e),
            }
        }
        Err(e) => println!("Error: {:#?}", e),
    }
}
