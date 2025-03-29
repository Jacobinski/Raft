use std::net::TcpListener;

enum Mode {
    Leader{
        // Volatile state (leader only)
        // This state is re-initialized after every election.
        next_index: Vec<u64>,  // The next log entry index to send to followers
        match_index: Vec<u64>, // Index of highest log entry known to be replicated by each follower
    },
    Follower,
    Candidate,
}

// TODO: Use the typestate pattern to provide compile-time guarantees about
// the state of the Server. https://cliffle.com/blog/rust-typestate/
pub struct Server {
    // Persistent State  TODO: Make this persistent
    // These fields are updated on stable storage before responding to messages.
    current_term: u64,      // Latest term seen by the server
    voted_for: Option<u64>, // Candidate which recieved vote in current term
    log: Vec<u64>,          // State machine log entries

    // Volatile State
    commit_index: u64, // Index of highest log entry known to be committed
    last_applied: u64, // Index of highest log entry applied to state machine
    mode: Mode,        // The mode of operation of the server

}

fn main() {
    // Send data to socket via `$ echo "foo" | nc localhost 8080`
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    match listener.accept() {
        Ok((_socket, addr)) => println!("new client: {addr:?}"),
        Err(e) => println!("cloudn't get client: {e:?}"),
    }
}
