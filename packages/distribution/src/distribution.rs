use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Uint128, Uint64};

#[cw_serde]
/// State of a single user's specific native rewards.
pub struct Distribution {
    // Membership issuer's user ID in membership protocol
    pub membership_issuer_user_id: Uint64,
    // User ID in membership protocol
    pub user_id: Uint64,
    /// The last global index at which the user's pending rewards were calculated
    pub user_index: Uint128,
    /// User's unclaimed rewards
    pub pending_rewards: Uint128,
}
