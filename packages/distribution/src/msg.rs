use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128, Uint64};

use crate::config::Config;

// ========== instantiate ==========

#[cw_serde]
pub struct InstantiateMsg {
    // Membership contract address, must be provided
    pub membership_contract_addr: String,
    // Default to sender
    pub admin_addr: Option<String>,
}

// ========== execute ==========

#[cw_serde]
pub enum ExecuteMsg {
    Enable(EnableMsg),
    Disable(DisableMsg),
    UpdateConfig(UpdateConfigMsg),

    // UpdateUserWeights(UpdateUserWeightsMsg),

    // Called only by membership contract when an user enabled its membership program
    // i.e. when EnabledMembershipMsg is call in membership contract
    SetupDistributionForNewMembership(SetupDistributionForNewMembershipMsg),

    // Called only by membership contract when an user first time become a member of a membership program
    // i.e. when user first time buy a membership
    SetupDistributionForNewMember(SetupDistributionForNewMemberMsg),

    // Called by membership contract when user buys / sells membership
    UpdateUserPendingReward(UpdateUserPendingRewardMsg),

    Distribute(DistributeMsg),

    // Anyone can call this to claim reward for a user
    // TODO: P1: use warp job to do it so users don't have to call it manually
    // TODO: P0: add batch claim rewards that claim many members of same membership issuer
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
}

#[cw_serde]
pub struct UpdateUserPendingRewardMsg {
    pub membership_issuer_user_id: Uint64,
    pub user_id: Uint64,
    pub user_previous_amount: Uint128,
}

#[cw_serde]
pub struct SetupDistributionForNewMembershipMsg {
    pub membership_issuer_user_id: Uint64,
}

#[cw_serde]
pub struct SetupDistributionForNewMemberMsg {
    pub membership_issuer_user_id: Uint64,
    pub user_id: Uint64,
}

#[cw_serde]
pub struct DistributeMsg {
    pub membership_issuer_user_id: Uint64,
    pub index_increment: Decimal,
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
    pub membership_issuer_user_id: Uint64,
    pub user_id: Uint64,
}

#[cw_serde]
pub struct UserRewardResponse {
    pub amount: Uint128,
}
