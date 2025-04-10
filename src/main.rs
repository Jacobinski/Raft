use std::net::TcpListener;
use crate::server::Server;
use raft::constants::LOCALHOST;

mod server;
mod watchdog;

fn main() {
    // TODO: Connect the server to the TCP socket
    let server = Server::new();
    println!("{:?}", server);

    // Send data to socket via `$ echo "foo" | nc localhost 8080`
    let listener = TcpListener::bind(LOCALHOST).unwrap();
    match listener.accept() {
        Ok((_socket, addr)) => println!("new client: {addr:?}"),
        Err(e) => println!("cloudn't get client: {e:?}"),
    }
}
