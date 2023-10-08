use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

// Used in membership map where key is the holder address, value is all memberships held by the holder
// So we can easily query all memberships of a holder
#[cw_serde]
pub struct Membership {
    // Membership's issuer address
    pub issuer_addr: Addr,
    // Number of membership held by the holder because each holder can buy multiple memberships
    pub amount: Uint128,
}
