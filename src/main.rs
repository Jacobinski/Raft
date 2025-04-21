use crate::server::Server;
use raft::{constants::LOCALHOST, server::AppendEntriesRequest};
use std::{fs::read, io::Read, net::TcpListener, vec};

mod server;
mod watchdog;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Connect the server to the TCP socket
    let server = Server::new();
    println!("{:?}", server);

    // Send data to socket via `$ echo "foo" | nc localhost 8080`
    let listener = TcpListener::bind(LOCALHOST).unwrap();
    match listener.accept() {
        Ok((mut socket, addr)) => {
            println!("new client: {addr:?}");
            let mut buffer = [0; 256];
            let read_bytes = socket.read(&mut buffer)?;
            let request: AppendEntriesRequest =
                serde_json::from_slice(&buffer[0..read_bytes]).unwrap();
            println!("{:?}", &request);
        }
        Err(e) => println!("cloudn't get client: {e:?}"),
    }

    Ok(())
}
