use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint64;

// In a thread, there are can be 1 or more askers but only a single answerer
// TODO: think about how we can support open thread, anyone can ask anyone
#[cw_serde]
pub struct Thread {
    // Thread ID, a global unique identifier that is monotonically increasing
    pub id: Uint64,
    // Thread title
    pub title: String,
    // Thread description
    pub description: String,
    // List of labels
    pub labels: Vec<String>,
    // The address of the key issuer that will answer all questions in this thread
    pub ask_to_addr: String,
}
