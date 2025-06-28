use serde::{Deserialize, Serialize};

/// Request represents all possible request messages
pub enum Request {
    AppendEntries(AppendEntriesRequest),
    RequestVote(RequestVoteRequest),
}

/// Response represents all possible response messages
pub enum Response {
    AppendEntries(AppendEntriesResponse),
    RequestVote(RequestVoteResponse),
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AppendEntriesRequest {
    pub term: u64,               // Leader's term
    pub leader_id: u64,          // Leader's ID
    pub previous_log_index: u64, // Index of log immediately preceeding new ones
    pub previous_log_term: u64,  // Term of the pervious_log_index entry
    pub entries: Vec<u64>, // Log entries to store (empty for heartbeat) TODO: Should this be an enum?
    pub leader_commit: u64, // Leader's commit_index
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AppendEntriesResponse {
    pub term: u64,     // The current term, used for updating the leader
    pub success: bool, // True if follower contains prev_log_index and prev_log_term
}

pub struct RequestVoteRequest {
    pub term: u64,           // Candidate's term
    pub candidate_id: u64,   // Candidate requesting vote
    pub last_log_index: u64, // Index of candidate's last log entry
    pub last_log_term: u64,  // Term of candidate's last log entry
}

pub struct RequestVoteResponse {
    pub term: u64,          // Current term, for candidate to update iself
    pub vote_granted: bool, // True if candidate recieves vote
}

impl AppendEntriesRequest {
    pub fn new() -> Self {
        AppendEntriesRequest {
            term: 0,
            leader_id: 0,
            previous_log_index: 0,
            previous_log_term: 0,
            entries: Vec::new(),
            leader_commit: 0,
        }
    }
}
