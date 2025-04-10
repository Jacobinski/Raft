use std::net::TcpListener;
use crate::server::Server;
use crate::constants::LOCALHOST;

mod server;
mod watchdog;
mod constants;

fn main() {
    let server = Server::new();
    println!("{:?}", server);

    // Send data to socket via `$ echo "foo" | nc localhost 8080`
    let listener = TcpListener::bind(LOCALHOST).unwrap();
    match listener.accept() {
        Ok((_socket, addr)) => println!("new client: {addr:?}"),
        Err(e) => println!("cloudn't get client: {e:?}"),
    }
}
