mod handler;
mod router;
mod server;
use crate::server::Server;

fn main() {
    // Start a server
    let server = Server::new("localhost:3000");
    //Run the server
    server.run();
    // println!("Hello, world!");
}
