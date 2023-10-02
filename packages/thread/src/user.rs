use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

use crate::key::Key;

#[cw_serde]
pub struct User {
    // User address
    pub addr: Addr,
    // User's social media handle, only exists if the register admin has linked the social media handle for the user
    pub social_media_handle: Option<String>,
    // Social media handle is required to issue key
    // Key issued by the user, only exists if the register admin has registered the key for the user
    pub issued_key: Option<Key>,
}
