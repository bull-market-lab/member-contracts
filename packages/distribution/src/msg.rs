use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Uint64};

use crate::config::Config;

// ========== instantiate ==========

#[cw_serde]
pub struct InstantiateMsg {
    // Membership contract address, must be provided
    pub membership_contract_addr: String,
    // Default to sender
    pub admin_addr: Option<String>,
    /// Optional minimum weight that the user must have to be eligible for rewards distributions
    pub minimum_eligible_weight: Option<Uint128>,
}

// ========== execute ==========

#[cw_serde]
pub enum ExecuteMsg {
    Enable(EnableMsg),
    Disable(DisableMsg),
    UpdateConfig(UpdateConfigMsg),

    UpdateUserWeights(UpdateUserWeightsMsg),

    // Called only by membership contract when an user enabled its membership program
    // i.e. when EnabledMembershipMsg is call in membership contract
    SetupDistributionForNewMembership(SetupDistributionForNewMembershipMsg),

    DistributeNative(DistributeNativeMsg),

    // Anyone can call this to claim reward for a user
    // TODO: P1: use warp job to do it so users don't have to call it manually
    // TODO: P0: add batch claim rewards
    ClaimReward(ClaimRewardsMsg),
}

#[cw_serde]
pub struct EnableMsg {}

#[cw_serde]
pub struct DisableMsg {}

#[cw_serde]
pub struct UpdateConfigMsg {
    pub admin_addr: Option<String>,
    pub membership_contract_addr: Option<String>,
    /// New minimum weight that the user must have to be eligible for rewards distributions
    pub minimum_eligible_weight: Option<Uint128>,
}

#[cw_serde]
pub struct UpdateUserWeightsMsg {
    pub membership_issuer_user_id: Uint64,
    pub user_id: Uint64,
    pub user_weight: Uint128,
}

#[cw_serde]
pub struct SetupDistributionForNewMembershipMsg {
    pub membership_issuer_user_id: Uint64,
}

#[cw_serde]
pub struct DistributeNativeMsg {
    pub membership_issuer_user_id: Uint64,
}

#[cw_serde]
pub struct ClaimRewardsMsg {
    pub membership_issuer_user_id: Uint64,
    pub user_id: Uint64,
}

// ========== query ==========

#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    QueryConfig(QueryConfigMsg),
    #[returns(UserRewardResponse)]
    QueryUserReward(QueryUserRewardMsg),
    // TODO: P0: pagination query all users reward
}

#[cw_serde]
pub struct QueryConfigMsg {}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct QueryUserRewardMsg {
    pub user_id: Uint64,
}

#[cw_serde]
pub struct UserRewardResponse {
    pub amount: Uint128,
}
