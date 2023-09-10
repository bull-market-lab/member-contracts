use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

use crate::key::Key;

#[cw_serde]
pub struct User {
    // User address
    pub addr: Addr,
    // User's social media handle, required to issue key
    pub social_media_handle: Option<String>,
    // Key issued by the user, only exists if the key register admin_addr has registered the key for the user
    pub issued_key: Option<Key>,
}
