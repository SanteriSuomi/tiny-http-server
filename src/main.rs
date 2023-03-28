fn main() {
    println!("Server starting...");
    match tiny_rust_server::run() {
        Ok(_) => println!("Server shutting down..."),
        Err(e) => println!("Error: {:#?}", e),
    }
}
