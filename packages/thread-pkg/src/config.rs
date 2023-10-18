use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, Uint64};
use shared_pkg::fee_share_config::FeeShareConfig;

#[cw_serde]
pub struct ThreadConfig {
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
}

#[cw_serde]
pub struct ProtocolFeeConfig {
    pub start_new_thread_fixed_cost: Uint128,
    pub ask_in_thread_fee_percentage: Uint64,
    // NOTE: answer has no cost
    pub reply_in_thread_fee_percentage: Uint64,
}

#[cw_serde]
pub struct FeeConfig {
    // Default Ask me fee in my 1 membership price percentage
    pub ask_fee_percentage_of_membership: Uint64,
    // How much to pay thread creator when someone ask in thread
    pub ask_fee_to_thread_creator_percentage_of_membership: Uint64,
    // Default Reply to me in my thread or my msg fee in my 1 membership price percentage
    pub reply_fee_percentage_of_membership: Uint64,
    // How much to pay thread creator when someone ask in thread
    pub reply_fee_to_thread_creator_percentage_of_membership: Uint64,
}

#[cw_serde]
pub struct Config {
    // Membership contract address, membership contract stores all user infos
    // Thread contract can be seen as an extension of membership contract that provides thread functionality
    // In the future, there could be other contracts on top of membership contract
    // e.g. one that provides off chain thread functions
    pub member_contract_addr: Addr,
    // Contract admin, able to upgrade contract
    pub admin_addr: Addr,
    // Enable or disable all user posting thread / ask / reply / answer
    pub enabled: bool,

    // Protocol fee collector, collects protocol fee
    pub protocol_fee_collector_addr: Addr,

    pub thread_config: ThreadConfig,

    pub protocol_fee_config: ProtocolFeeConfig,

    pub default_fee_config: FeeConfig,

    pub default_fee_share_config: FeeShareConfig,
}
