use clap::Parser;
use raft::constants::LOCALHOST;
use raft::message::AppendEntriesRequest;
use tokio::io::{AsyncWriteExt, Result};
use tokio::net::TcpStream;

#[derive(Parser)]
struct Args {
    /// Entries to append to log
    #[arg(long, short)]
    entries: Vec<u64>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut client = TcpStream::connect(LOCALHOST).await.unwrap();
    // TODO: Convert this to a builder pattern
    let mut req = AppendEntriesRequest::new();
    req.entries = args.entries;
    let bytes = serde_json::to_vec(&req)?;
    client.write_all(&bytes).await?;
    Ok(())
}
