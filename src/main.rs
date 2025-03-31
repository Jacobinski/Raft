use std::fmt::Debug;

// ServerMode is a typestate trait that controls the API of the server
// https://cliffle.com/blog/rust-typestate
trait ServerMode: Debug {}

// An uninitialized node has no peers.
// TODO: ... or should we allow uninitialized nodes to run solo?
#[derive(Debug)]
struct Uninitialized;
impl ServerMode for Uninitialized {}

// A follower node accepts RPCs from the leader.
#[derive(Debug)]
struct Follower;
impl ServerMode for Follower {}

// A candidate node is attempting to become a leader.
#[derive(Debug)]
struct Candidate;
impl ServerMode for Candidate {}

// A leader node controls the consensus cluster.
#[derive(Debug)]
struct Leader {
    // This state is re-initialized after every election.
    next_index: Vec<u64>,  // The next log entry index to send to followers
    match_index: Vec<u64>, // Index of highest log entry known to be replicated by each follower
}
impl ServerMode for Leader {}

// TODO: Use the typestate pattern to provide compile-time guarantees about
// the state of the Server. https://cliffle.com/blog/rust-typestate/
#[derive(Debug)]
pub struct Server<S: ServerMode> {
    // Persistent State  TODO: Make this persistent
    // These fields are updated on stable storage before responding to messages.
    current_term: u64,      // Latest term seen by the server
    voted_for: Option<u64>, // Candidate which recieved vote in current term
    log: Vec<u64>,          // State machine log entries

    // Volatile State
    commit_index: u64, // Index of highest log entry known to be committed
    last_applied: u64, // Index of highest log entry applied to state machine
    mode: S,           // The mode of operation of the server

    // Additional fields not mentioned in paper
    peers: Vec<Box<dyn Node>>, // Peers which can accept raft RPCs
}

pub struct AppendEntriesRequest {
    term: u64,               // Leader's term
    leader_id: u64,          // Leader's ID
    previous_log_index: u64, // Index of log immediately preceeding new ones
    previous_log_term: u64,  // Term of the pervious_log_index entry
    entries: Vec<u64>,       // Log entries to store (empty for heartbeat) TODO: Should this be an enum?
    leader_commit: u64,      // Leader's commit_index
}

pub struct AppendEntriesResponse {
    term: u64,    // The current term, used for updating the leader
    success: u64, // True if follower contains prev_log_index and prev_log_term
}

pub struct RequestVoteRequest {
    term: u64,           // Candidate's term
    candidate_id: u64,   // Candidate requesting vote
    last_log_index: u64, // Index of candidate's last log entry
    last_log_term: u64,  // Term of candidate's last log entry
}

pub struct RequestVoteResponse {
    term: u64,          // Current term, for candidate to update iself
    vote_granted: bool, // True if candidate recieves vote
}

/// The public interface of a Raft node.
trait Node: Debug {
    fn append_entries(&self, req: AppendEntriesRequest) -> AppendEntriesResponse;
    fn request_vote(&self, req: RequestVoteRequest) -> RequestVoteResponse;
}

impl Server<Uninitialized> {
    fn new() -> Self {
        Server{
            current_term: 0,
            voted_for: None,
            log: Vec::new(),
            commit_index: 0,
            last_applied: 0,
            mode: Uninitialized{},
            peers: Vec::new(), // TODO: Should uninitialized contain peers?
        }
    }
}

impl<S: ServerMode> Node for Server<S> {
    fn append_entries(&self, req: AppendEntriesRequest) -> AppendEntriesResponse {
        todo!()
    }

    fn request_vote(&self, req: RequestVoteRequest) -> RequestVoteResponse {
        todo!()
    }
}

fn main() {
    let server = Server::new();
    println!("{:?}", server)
}
