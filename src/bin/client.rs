use raft::constants::LOCALHOST;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut client = TcpStream::connect(LOCALHOST).await.unwrap();
}
