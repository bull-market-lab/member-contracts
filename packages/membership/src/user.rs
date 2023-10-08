use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};

#[cw_serde]
pub struct User {
    // User address
    pub addr: Addr,
    // User's social media handle, only exists if the register admin has linked the social media handle for the user
    pub social_media_handle: Option<String>,
    // Social media handle is required to issue key
    // Membership issued by the user, only exists if the register admin has registered the key for the user
    pub membership_enabled: bool,

    // Membership trading fee in my 1 key price percentage
    // Split according to key_trading_fee_share_config across protocol, key issuer and key holders
    // Use protocol default if unset
    pub trading_fee_percentage_of_membership: Option<Uint64>,

    // Fee config for key trading, if unset use protocol default key trading fee config
    // Revenue share percentage for membership issuer
    pub share_to_issuer_percentage: Option<Uint64>,
    // Revenue share percentage for all members
    pub share_to_all_members_percentage: Option<Uint64>,
}
