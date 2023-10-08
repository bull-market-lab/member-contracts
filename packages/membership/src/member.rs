use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

// Used in member map where key is the issuer address, value is all its members
// So we can easily query all members by an issuer
#[cw_serde]
pub struct Member {
    // Membership's holder address
    pub holder_addr: Addr,
    // Number of membership held by the holder because each holder can buy multiple memberships
    pub amount: Uint128,
}
