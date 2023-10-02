use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct Key {
    // Total number of keys issued
    pub supply: Uint128,
}
