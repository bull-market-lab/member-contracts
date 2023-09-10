use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct UserHolding {
    // Key's issuer address
    pub issuer_addr: Addr,
    // Number of keys held by the holder
    pub amount: Uint128,
}
