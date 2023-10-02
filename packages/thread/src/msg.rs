use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128, Uint64};

use crate::{
    config::Config,
    key::{KeyTradingFeeConfig, ThreadFeeConfig},
    key_holder::KeyHolder,
    user::User,
    user_holding::UserHolding,
};

// ========== instantiate ==========

#[cw_serde]
pub struct InstantiateMsg {
    // Default to sender
    pub admin_addr: Option<String>,
    // Default to sender
    pub registration_admin_addr: Option<String>,
    // Default to sender
    pub protocol_fee_collector_addr: Option<String>,
    // Default to uluna
    // TODO: use noble USDC
    pub fee_denom: Option<String>,
    // Default to 50
    pub max_thread_title_length: Option<Uint64>,
    // Default to 500
    pub max_thread_msg_length: Option<Uint64>,
}

// ========== execute ==========

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig(UpdateConfigMsg),

    // Anyone can register an account
    // But without registering a key they can only buy and sell other people's keys but not issue their own keys
    Register(),

    // Only register admin can link social media for user
    LinkSocialMedia(LinkSocialMediaMsg),

    // Only register admin can register key for user
    // User must link social media first to be eligible for key registration to prevent impersonation
    // This will initialize the user's key and set the supply to 1 owned by the user
    // After that anyone can buy / sell user's key
    RegisterKey(RegisterKeyMsg),

    // Only key issuer can update its key trading fee config
    UpdateTradingFeeConfig(UpdateTradingFeeConfigMsg),

    // Only key issuer can update its thread fee config
    UpdateThreadFeeConfig(UpdateThreadFeeConfigMsg),

    // Anyone can buy key
    BuyKey(BuyKeyMsg),

    // Anyone can sell key if they have it
    SellKey(SellKeyMsg),

    // Key holder can ask question to key issuer in an existing thread or a new thread
    Ask(AskMsg),

    // Key issuer can answer question to key holder in an existing thread
    Answer(AnswerMsg),
    // TODO: new msg to support withdraw question after key issuer not answer for a while, this will send fee back to user
    // TODO: new msg to support open question, anyone can answer
}

#[cw_serde]
pub struct UpdateConfigMsg {
    pub admin_addr: Option<String>,
    pub registration_admin_addr: Option<String>,
    pub protocol_fee_collector_addr: Option<String>,
    pub fee_denom: Option<String>,
    pub max_thread_title_length: Option<Uint64>,
    pub max_thread_msg_length: Option<Uint64>,
}

#[cw_serde]
pub struct LinkSocialMediaMsg {
    pub user_addr: Addr,
    pub social_media_handle: String,
}

#[cw_serde]
pub struct RegisterKeyMsg {
    pub user_addr: Addr,
    pub key_trading_fee_config: KeyTradingFeeConfig,
    pub thread_fee_config: ThreadFeeConfig,
}

#[cw_serde]
pub struct UpdateTradingFeeConfigMsg {
    pub key_issuer_addr: Addr,
    pub key_trading_fee_config: KeyTradingFeeConfig,
}

#[cw_serde]
pub struct UpdateThreadFeeConfigMsg {
    pub key_issuer_addr: Addr,
    pub thread_fee_config: ThreadFeeConfig,
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

#[cw_serde]
pub struct AskMsg {
    // The address of the key issuer that the user wants to ask question to
    pub ask_to_addr: Addr,
    // New to start a new thread, default to false
    pub start_new_thread: Option<bool>,
    // Thread ID to ask question in
    // If start_new_thread is false, this field must be filled
    // Else start_new_thread is true, this field will be ignored
    pub thread_id: Option<Uint64>,
    // If start_new_thread is true, this field must be filled
    // Else start_new_thread is false, this field will be ignored
    pub thread_title: Option<String>,
    // If start_new_thread is true, this field must be filled
    // Else start_new_thread is false, this field will be ignored
    pub thread_description: Option<String>,
    // If start_new_thread is true, this field must be filled
    // Else start_new_thread is false, this field will be ignored
    pub thread_labels: Option<Vec<String>>,
    // Question content
    pub content: String,
}

#[cw_serde]
pub struct AnswerMsg {
    // Thread ID to answer question in
    pub thread_id: Uint64,
    // Answer must be replying to a specific question in a thread
    pub question_id: Uint64,
    // Answer content
    pub content: String,
}

// ========== query ==========

#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    QueryConfig(QueryConfigMsg),

    #[returns(UserResponse)]
    QueryUser(QueryUserMsg),

    // Returns all users holding the key, with pagination
    #[returns(KeyHoldersResponse)]
    QueryKeyHolders(QueryKeyHoldersMsg),

    // Returns all keys user currently holds, with pagination
    #[returns(UserHoldingsResponse)]
    QueryUserHoldings(QueryUserHoldingsMsg),

    // QuerySimulateBuyKey calculates the price and fee
    #[returns(SimulateBuyKeyResponse)]
    QuerySimulateBuyKey(QuerySimulateBuyKeyMsg),

    // QuerySimulateSellKey calculates the price and fee
    #[returns(SimulateSellKeyResponse)]
    QuerySimulateSellKey(QuerySimulateSellKeyMsg),

    // QuerySimulateAsk calculates the fee needed to ask a question
    #[returns(SimulateAskResponse)]
    QuerySimulateAsk(QuerySimulateAskMsg),
}

#[cw_serde]
pub struct QueryConfigMsg {}

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
pub struct QuerySimulateBuyKeyMsg {
    pub key_issuer_addr: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub struct SimulateBuyKeyResponse {
    // Price is total price for buy amount of key, not the price per key
    pub price: Uint128,
    // Fee paid to protocol
    pub protocol_fee: Uint128,
    // Fee paid to key issuer
    pub key_issuer_fee: Uint128,
    // Fee paid to all key holders
    pub key_holder_fee: Uint128,
    // Price + protocol fee + key issuer fee + key holder fee
    pub total_needed_from_user: Uint128,
}

#[cw_serde]
pub struct QuerySimulateSellKeyMsg {
    pub key_issuer_addr: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub struct SimulateSellKeyResponse {
    // Price is total price for sell amount of key, not the price per key
    pub price: Uint128,
    // Fee paid to protocol
    pub protocol_fee: Uint128,
    // Fee paid to key issuer
    pub key_issuer_fee: Uint128,
    // Fee paid to all key holders
    pub key_holder_fee: Uint128,
    // Protocol fee + key issuer fee + key holder fee
    pub total_needed_from_user: Uint128,
}

#[cw_serde]
pub struct QuerySimulateAskMsg {
    // The address of the key issuer that the user wants to ask question to
    pub ask_to_addr: Addr,
    // Number of characters in question content
    pub content_len: Uint128,
}

#[cw_serde]
pub struct SimulateAskResponse {
    // Fee paid to protocol
    pub protocol_fee: Uint128,
    // Fee paid to key issuer
    pub key_issuer_fee: Uint128,
    // Fee paid to all key holders
    pub key_holder_fee: Uint128,
    // Protocol fee + key issuer fee + key holder fee
    pub total_needed_from_user: Uint128,
}
