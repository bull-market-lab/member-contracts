use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};

use crate::config::FeeShareConfig;

#[cw_serde]
pub struct User {
    // User address
    pub addr: Addr,
    // User's social media handle, only exists if the register admin has linked the social media handle for the user
    pub social_media_handle: Option<String>,
    // Social media handle is required to issue key
    // Key issued by the user, only exists if the register admin has registered the key for the user
    pub issued_key: bool,

    // Key trading fee in my 1 key price percentage
    // Split according to key_trading_fee_share_config across protocol, key issuer and key holders
    // Use protocol default if unset
    pub trading_fee_percentage_of_key: Option<Uint64>,
    // Ask me fee in my 1 key price percentage
    // Split according to thread_fee_share_config across protocol, key issuer and key holders
    // Use protocol default if unset
    pub ask_fee_percentage_of_key: Option<Uint64>,
    // Reply to me in my thread or my msg fee in my 1 key price percentage
    // Split according to thread_fee_share_config across protocol, key issuer and key holders
    // Use protocol default if unset
    pub reply_fee_percentage_of_key: Option<Uint64>,

    // Fee config for key trading, if unset use protocol default key trading fee config
    pub key_trading_fee_share_config: Option<FeeShareConfig>,
    // Fee config for thread, if unset use protocol default thread fee config
    pub thread_fee_share_config: Option<FeeShareConfig>,
    // TODO: P1: user can choose their own fee denom
}
