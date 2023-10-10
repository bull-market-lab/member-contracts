use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint64;

// User doesn't need to sign up to use the thread, as long as user sign up with the membership contract
// It has access to the thread contract.
// User struct here is only storing thread specific user config, it can be seen as an extension of the user struct in membership contract
#[cw_serde]
pub struct UserConfig {
    // User ID in the membership contract
    pub id: Uint64,

    // Ask me fee in my 1 membership price percentage
    // Split according to thread_fee_share_config across protocol, membership issuer and membership holders
    // Use protocol default if unset
    pub ask_fee_percentage_of_membership: Option<Uint64>,
    // When i'm the thread creator, how much asker needs to pay me to ask in my thread
    // Use protocol default if unset
    pub ask_fee_to_thread_creator_percentage_of_membership: Option<Uint64>,
    // Reply to me in my thread or my msg fee in my 1 membership price percentage
    // Split according to thread_fee_share_config across protocol, membership issuer and membership holders
    // Use protocol default if unset
    pub reply_fee_percentage_of_membership: Option<Uint64>,

    // Fee config for thread, if unset use protocol default thread fee config
    // Revenue share percentage for membership issuer
    pub share_to_issuer_percentage: Option<Uint64>,
    // Revenue share percentage for all members
    pub share_to_all_members_percentage: Option<Uint64>,
}
