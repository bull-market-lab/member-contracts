use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};

use crate::config::FeeShareConfig;

#[cw_serde]
pub struct User {
    // User address
    pub addr: Addr,

    // Ask me fee in my 1 key price percentage
    // Split according to thread_fee_share_config across protocol, key issuer and key holders
    // Use protocol default if unset
    pub ask_fee_percentage_of_key: Option<Uint64>,
    // When i'm the thread creator, how much asker needs to pay me to ask in my thread
    // Use protocol default if unset
    pub ask_fee_to_thread_creator_percentage_of_key: Option<Uint64>,
    // Reply to me in my thread or my msg fee in my 1 key price percentage
    // Split according to thread_fee_share_config across protocol, key issuer and key holders
    // Use protocol default if unset
    pub reply_fee_percentage_of_key: Option<Uint64>,

    // Fee config for key trading, if unset use protocol default key trading fee config
    pub key_trading_fee_share_config: Option<FeeShareConfig>,
    // Fee config for thread, if unset use protocol default thread fee config
    pub thread_fee_share_config: Option<FeeShareConfig>,
    // TODO: P1: user can choose their own fee denom
    // TODO: P1: add a mode where user can force others only able to buy up to 1 key
    // This will make the price fluctuate that much
}
