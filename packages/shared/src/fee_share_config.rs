use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint64;

#[cw_serde]
pub struct FeeShareConfig {
    // Revenue share percentage for membership issuer
    pub share_to_issuer_percentage: Uint64,
    // Revenue share percentage for all members
    pub share_to_all_members_percentage: Uint64,
}
