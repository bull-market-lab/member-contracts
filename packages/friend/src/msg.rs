use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

use crate::{config::Config, key_holder::KeyHolder, user::User, user_holding::UserHolding};

// ========== instantiate ==========

#[cw_serde]
pub struct InstantiateMsg {
    pub admin_addr: Option<String>,
    pub key_register_admin_addr: Option<String>,
    pub protocol_fee_collector_addr: Option<String>,
    pub fee_denom: Option<String>,
    pub protocol_fee_percentage: Uint128,
    pub key_issuer_fee_percentage: Uint128,
}

// ========== execute ==========

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig(UpdateConfigMsg),

    // Anyone can register an account
    // But without registering a key they can only buy and sell other people's keys but not issue their own keys
    Register(),

    // Only key register admin_addr can register key for an account
    RegisterSocialMediaAndKey(RegisterSocialMediaAndKeyMsg),

    // Anyone can buy key
    BuyKey(BuyKeyMsg),

    // Anyone can sell key if they have it
    SellKey(SellKeyMsg),
}

#[cw_serde]
pub struct UpdateConfigMsg {
    pub admin_addr: Option<String>,
    pub key_register_admin_addr: Option<String>,
    pub protocol_fee_collector_addr: Option<String>,
    pub fee_denom: Option<String>,
    pub protocol_fee_percentage: Option<Uint128>,
    pub key_issuer_fee_percentage: Option<Uint128>,
}

#[cw_serde]
pub struct RegisterSocialMediaAndKeyMsg {
    pub user_addr: Addr,
    pub social_media_handle: String,
}

#[cw_serde]
pub struct BuyKeyMsg {
    pub key_issuer_addr: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub struct SellKeyMsg {
    pub key_issuer_addr: Addr,
    pub amount: Uint128,
}

// ========== query ==========

#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    QueryConfig(),

    #[returns(UserResponse)]
    QueryUser(QueryUserMsg),

    #[returns(KeyHoldersResponse)]
    QueryKeyHolders(QueryKeyHoldersMsg),

    #[returns(UserHoldingsResponse)]
    QueryUserHoldings(QueryUserHoldingsMsg),

    #[returns(KeySupplyResponse)]
    QueryKeySupply(QueryKeySupplyMsg),

    #[returns(SimulateBuyKeyResponse)]
    QuerySimulateBuyKey(QuerySimulateBuyKeyMsg),

    #[returns(SimulateSellKeyResponse)]
    QuerySimulateSellKey(QuerySimulateSellKeyMsg),
}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct QueryUserMsg {
    pub user_addr: Addr,
}

#[cw_serde]
pub struct UserResponse {
    pub user: User,
}

#[cw_serde]
pub struct QueryKeyHoldersMsg {
    pub key_issuer_addr: Addr,
    pub start_after_user_addr: Option<Addr>,
    pub limit: Option<u32>,
}

#[cw_serde]
pub struct KeyHoldersResponse {
    pub key_holders: Vec<KeyHolder>,
    pub total_count: usize,
}

#[cw_serde]
pub struct QueryUserHoldingsMsg {
    pub user_addr: Addr,
    pub start_after_key_issuer_addr: Option<Addr>,
    pub limit: Option<u32>,
}

#[cw_serde]
pub struct UserHoldingsResponse {
    pub user_holdings: Vec<UserHolding>,
    pub total_count: usize,
}

#[cw_serde]
pub struct QueryKeySupplyMsg {
    pub key_issuer_addr: Addr,
}

#[cw_serde]
pub struct KeySupplyResponse {
    pub supply: Uint128,
}

#[cw_serde]
pub struct QuerySimulateBuyKeyMsg {
    pub key_issuer_addr: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub struct SimulateBuyKeyResponse {
    // Price is total price, not the price per key
    pub price: Uint128,
    pub protocol_fee: Uint128,
    pub key_issuer_fee: Uint128,
    // Price + protocol fee + key issuer fee
    pub total_needed_from_user: Uint128,
}

#[cw_serde]
pub struct QuerySimulateSellKeyMsg {
    pub key_issuer_addr: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub struct SimulateSellKeyResponse {
    // Price is total price, not the price per key
    pub price: Uint128,
    pub protocol_fee: Uint128,
    pub key_issuer_fee: Uint128,
    // Protocol fee + key issuer fee
    pub total_needed_from_user: Uint128,
}
