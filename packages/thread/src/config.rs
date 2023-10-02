use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};

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
    // Max length of QA thread title
    pub max_qa_thread_title_length: Uint64,
    // Max length of QA thread msg content
    pub max_qa_thread_msg_length: Uint64,
}
