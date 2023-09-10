use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct Config {
    // Contract admin, able to upgrade contract
    pub admin: Addr,
    // Key register admin, able to register key for existing users
    pub key_register_admin: Addr,
    // Fee collector, collects protocol fee
    pub fee_collector: Addr,
    // Protocol fee percentage
    pub protocol_fee_percentage: Uint128,
    // Owner fee percentage
    pub key_issuer_fee_percentage: Uint128,
}
