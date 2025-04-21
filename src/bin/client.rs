use raft::constants::LOCALHOST;
use raft::server::AppendEntriesRequest;
use tokio::io::{AsyncWriteExt, Result};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = TcpStream::connect(LOCALHOST).await.unwrap();
    let req = AppendEntriesRequest::new();
    let bytes = serde_json::to_vec(&req)?;
    client.write_all(&bytes).await?;
    Ok(())
}
