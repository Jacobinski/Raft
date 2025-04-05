use std::net::TcpListener;
use crate::server::Server;

mod server;
mod watchdog;

fn main() {
    let server = Server::new();
    println!("{:?}", server);

    // Send data to socket via `$ echo "foo" | nc localhost 8080`
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    match listener.accept() {
        Ok((_socket, addr)) => println!("new client: {addr:?}"),
        Err(e) => println!("cloudn't get client: {e:?}"),
    }
}
