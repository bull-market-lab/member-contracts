use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct Config {
    // Contract admin, able to upgrade contract
    pub admin_addr: Addr,
    // Membership contract address, membership contract stores all user infos
    // Thread contract can be seen as an extension of membership contract that provides thread functionality
    // In the future, there could be other contracts on top of membership contract
    // e.g. one that provides off chain thread functions
    pub membership_contract_addr: Addr,
    // Enable or disable all user posting thread / ask / reply / answer
    pub enabled: bool,
    // /// Minimal weight that the member must have to be eligible for member fee distributions
    // pub minimum_eligible_weight: Uint128,
}
