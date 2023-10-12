use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};

#[cw_serde]
pub struct ProtocolFeeConfig {
    pub membership_trading_fee_percentage: Uint64,
}

#[cw_serde]
pub struct FeeConfig {
    // Denom of fee, e.g. uluna, once set cannot change
    pub fee_denom: String,
    // Default membership trading fee in my 1 membership price percentage
    pub trading_fee_percentage_of_membership: Uint64,
}

#[cw_serde]
pub struct FeeShareConfig {
    // Revenue share percentage for membership issuer
    pub share_to_issuer_percentage: Uint64,
    // Revenue share percentage for all members
    pub share_to_all_members_percentage: Uint64,
}

#[cw_serde]
pub struct Config {
    // Contract admin, able to upgrade contract
    pub admin_addr: Addr,
    // Distribution contract address, used to distribute fees to all members
    // This should be set right after distribution contract is deployed
    // We don't set it at membership contract instantiation because we deploy membership first then distribution
    // If unset then all members fee will stay at membership contract, but this should never happen
    pub distribution_contract_addr: Option<Addr>,
    // Enable or disable all user facing functions, signup, membership, trading
    pub enabled: bool,
    // If true then anyone can sign up, but link social media and register membership still needs registration admin
    // If false then only registration admin can sign up for new users
    // TODO: P1: add new referral
    // TODO: P1: support transfer ownership
    // TODO: P0: separate membership and thread to 2 contracts
    pub enable_open_registration: bool,
    // Registration admin, able to register membership for existing users
    pub registration_admin_addr: Addr,
    // Protocol fee collector, collects protocol fee
    pub protocol_fee_collector_addr: Addr,
    pub protocol_fee_config: ProtocolFeeConfig,
    pub default_fee_config: FeeConfig,
    pub default_fee_share_config: FeeShareConfig,
}
