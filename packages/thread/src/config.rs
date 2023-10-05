use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, Uint64};

#[cw_serde]
pub struct ProtocolFeeConfig {
    pub key_trading_fee_percentage: Uint64,
    pub start_new_thread_fixed_cost: Uint128,
    pub ask_in_thread_fee_percentage: Uint64,
    // NOTE: answer has no cost
    pub reply_in_thread_fee_percentage: Uint64,
}

#[cw_serde]
pub struct FeeShareConfig {
    // Key issuer fee percentage
    pub key_issuer_fee_percentage: Uint64,
    // Key holder fee percentage
    pub key_holder_fee_percentage: Uint64,
}

#[cw_serde]
pub struct Config {
    // Contract admin, able to upgrade contract
    pub admin_addr: Addr,
    // Registration admin, able to register key for existing users
    pub registration_admin_addr: Addr,
    // Protocol fee collector, collects protocol fee
    pub protocol_fee_collector_addr: Addr,
    // Denom of fee, e.g. uluna
    pub fee_denom: String,
    // Max length of thread title in characters
    pub max_thread_title_length: Uint64,
    // Max length of thread description in characters
    pub max_thread_description_length: Uint64,
    // Max length of a single thread label in characters
    pub max_thread_label_length: Uint64,
    // Max number of thread labels
    pub max_number_of_thread_labels: Uint64,
    // Max length of thread msg content in characters
    pub max_thread_msg_length: Uint64,

    // Protocol fee config
    pub protocol_fee_config: ProtocolFeeConfig,

    // Default key trading fee in my 1 key price percentage
    pub default_trading_fee_percentage_of_key: Uint64,
    // Default Ask me fee in my 1 key price percentage
    pub default_ask_fee_percentage_of_key: Uint64,
    // How much to pay thread creator when someone ask in thread
    pub default_ask_fee_to_thread_creator_percentage_of_key: Uint64,
    // Default Reply to me in my thread or my msg fee in my 1 key price percentage
    pub default_reply_fee_percentage_of_key: Uint64,

    // Default key trading fee config
    pub default_key_trading_fee_share_config: FeeShareConfig,
    // Default thread fee config
    pub default_thread_fee_share_config: FeeShareConfig,
}
