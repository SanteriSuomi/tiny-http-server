use tiny_rust_server::communication::router::Router;
use tiny_rust_server::server::Server;

fn main() {
    match Server::new((127, 0, 0, 1), 5000) {
        Ok(mut server) => {
            // Serve static files from the "public" folder at project root
            server.serve_static("public");

            // Define a new router instance
            let mut router = Router::new("/test");
            // Add a middleware to the router (executed before every request)
            router.middleware(|_mid, req| {
                println!("MIDDLEWARE executed");
                req.headers
                    .insert("User-Agent".to_string(), "Testi".to_string());
            });
            // Add a route to the router, route will be router base path + router path
            router.route("", "GET", |req, res| {
                println!("GET \nRequest: {:#?} \nResponse: {:#?}", req, res);
                res.set_content("Hello");
            });
            // Finally, register router with the server
            server.router(router);

            println!("Server started");
            match server.run() {
                Ok(_) => println!("Server shutdown"),
                Err(e) => println!("Server Error: {:#?}", e),
            }
        }
        Err(e) => println!("Error: {:#?}", e),
    }
}
