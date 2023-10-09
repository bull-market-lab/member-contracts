use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint128, Uint64};

#[cw_serde]
/// State of a single user's specific native rewards.
pub struct Distribution {
    // User ID in membership protocol
    pub user_id: Uint64,
    /// The last global index at which the user's pending rewards were calculated
    pub user_index: Decimal,
    /// User's unclaimed rewards
    pub pending_rewards: Uint128,
    /// Real weight should just equal to the number of membership user holds in membership contract
    pub real_weight: Uint128,
    // /// Effective user weights are their weights when taking into account minimum eligible weight
    // /// for rewards.
    // /// This weight will be the same as user's real weight if they're over the minimum eligible weight,
    // /// or 0 if they are under the minimum.
    // pub effective_weight: Uint128,
}
