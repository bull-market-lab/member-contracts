use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct Config {
    // Contract admin_addr, able to upgrade contract
    pub admin_addr: Addr,
    // Key register admin_addr, able to register key for existing users
    pub key_register_admin_addr: Addr,
    // Protocol fee collector, collects protocol fee
    pub protocol_fee_collector_addr: Addr,
    // Denom of fee, e.g. uluna
    pub fee_denom: String,
    // Protocol fee percentage
    pub protocol_fee_percentage: Uint128,
    // Owner fee percentage
    pub key_issuer_fee_percentage: Uint128,
}
