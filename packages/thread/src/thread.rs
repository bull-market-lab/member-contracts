use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Uint64, Addr};

// TODO: P0: add comment
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
    // The address of the thread creator
    pub creator_addr: Addr,
}
