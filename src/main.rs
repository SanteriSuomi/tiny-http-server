use tiny_rust_server::client::server::Server;

fn main() {
    match Server::new(5000) {
        Ok(mut server) => {
            server.serve_static("public");

            server.route("/test", "GET", |req, res| {
                println!("GET Request: {:#?}", req);
                res.set_status_code("200");
                res.set_status_message("OK");
                res.set_content("Hello World!");
                println!("Response: {:#?}", res);
            });

            server.route("/test", "POST", |req, _res| {
                println!("POST Request: {:#?}", req);
            });
            match server.run() {
                Ok(_) => println!("Server running..."),
                Err(e) => println!("Server Error: {:#?}", e),
            }
        }
        Err(e) => println!("Error: {:#?}", e),
    }
}
