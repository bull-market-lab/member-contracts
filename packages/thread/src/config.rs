use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, Uint64};

#[cw_serde]
pub struct Config {
    // Membership contract address, membership contract stores all user infos
    // Thread contract can be seen as an extension of membership contract that provides thread functionality
    // In the future, there could be other contracts on top of membership contract
    // e.g. one that provides off chain thread functions
    pub membership_contract_addr: Addr,
    // Contract admin, able to upgrade contract
    pub admin_addr: Addr,
    // Enable or disable all user posting thread / ask / reply / answer
    pub enabled: bool,

    // Protocol fee collector, collects protocol fee
    pub protocol_fee_collector_addr: Addr,

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
    pub protocol_fee_start_new_thread_fixed_cost: Uint128,
    pub protocol_fee_ask_in_thread_fee_percentage: Uint64,
    // NOTE: answer has no cost
    pub protocol_fee_reply_in_thread_fee_percentage: Uint64,

    // Default Ask me fee in my 1 membership price percentage
    pub default_ask_fee_percentage_of_membership: Uint64,
    // How much to pay thread creator when someone ask in thread
    pub default_ask_fee_to_thread_creator_percentage_of_membership: Uint64,
    // Default Reply to me in my thread or my msg fee in my 1 membership price percentage
    pub default_reply_fee_percentage_of_membership: Uint64,
    // How much to pay thread creator when someone ask in thread
    pub default_reply_fee_to_thread_creator_percentage_of_membership: Uint64,

    // Default thread fee share config
    // Revenue share percentage for membership issuer
    pub default_share_to_issuer_percentage: Uint64,
    // Revenue share percentage for all members
    pub default_share_to_all_members_percentage: Uint64,
}
