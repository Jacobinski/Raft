use crate::server::Server;
use raft::{constants::LOCALHOST, server::AppendEntriesRequest};
use tokio::{io::AsyncReadExt, net::TcpListener};

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Connect the server to the TCP socket
    let mut server = Server::new();
    println!("{:?}", server);

    // Send data to socket via `$ echo "foo" | nc localhost 8080`
    let listener = TcpListener::bind(LOCALHOST).await?;
    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                println!("new client: {addr:?}");
                let mut buffer = Vec::with_capacity(256);
                let read_bytes = socket.read_buf(&mut buffer).await?;
                let mut request: AppendEntriesRequest =
                    serde_json::from_slice(&buffer[0..read_bytes]).unwrap();

                println!("request.entries: {:?}", &request.entries);
                server.log.append(&mut request.entries);
                println!("server.log: {:?}", &server.log);
            }
            Err(e) => println!("cloudn't get client: {e:?}"),
        }
    }
}
