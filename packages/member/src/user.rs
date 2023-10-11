use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, Uint64};

// Used in membership map where membership is the holder address, value is all memberships held by the holder
// So we can easily query all memberships of a holder
#[cw_serde]
pub struct Membership {
    // Membership issuer's user ID
    pub issuer_user_id: Uint64,
    // Number of membership held by the holder because each holder can buy multiple memberships
    pub amount: Uint128,
}

// Used in member map where membership is the issuer address, value is all its members
// So we can easily query all members by an issuer
#[cw_serde]
pub struct Member {
    // Member's user ID
    pub member_user_id: Uint64,
    // Number of membership held by the holder because each holder can buy multiple memberships
    pub amount: Uint128,
}

#[cw_serde]
pub struct MembershipIssuedByMe {
    // Number of memberships issued by the user
    pub membership_supply: Uint128,
    // Number of members who hold the membership issued by the user
    // This could be smaller than membership_supply because each member can hold multiple memberships
    pub member_count: Uint128,
}

#[cw_serde]
pub struct UserConfig {
    // Membership trading fee in my 1 membership price percentage
    // Split according to membership_trading_fee_share_config across protocol, membership issuer and membership holders
    // Use protocol default if unset
    pub trading_fee_percentage_of_membership: Option<Uint64>,

    // Fee config for membership trading, if unset use protocol default membership trading fee config
    // Revenue share percentage for membership issuer
    pub share_to_issuer_percentage: Option<Uint64>,
    // Revenue share percentage for all members
    pub share_to_all_members_percentage: Option<Uint64>,
}

#[cw_serde]
pub struct User {
    // User ID, a global unique identifier that is monotonically increasing
    pub id: Uint64,
    // User address
    // TODO: P2: support cold wallet address and hot wallet address
    // So user can use cold wallet to buy / sell key, hot wallet to post thread
    pub addr: Addr,
    // User's social media handle, only exists if the register admin has linked the social media handle for the user
    pub social_media_handle: Option<String>,

    // User customized fee config
    pub config: UserConfig,

    // user_member_count = user is what member of how many other user's memberships
    // User can join others membership without issuing its own membership
    // We store this field here because cosmwasm doesn't support O(1) getting size of map
    pub user_member_count: Uint128,

    // Social media handle is required to issue its own membership
    // Membership issued by the user, only exists if the register admin has registered the membership for the user
    pub membership_issued_by_me: Option<MembershipIssuedByMe>,
    // TODO: P2: user can set their own fee denom, but this cannot be changed after set (not easily)
    // pub fee_denom: String,
}
