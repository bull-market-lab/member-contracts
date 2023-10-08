use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

use membership::{config::Config, user::User};

pub const DEFAULT_QUERY_LIMIT: u32 = 5;
pub const MAX_QUERY_LIMIT: u32 = 25;

pub const CONFIG: Item<Config> = Item::new("CONFIG");

// Membership is user address, value is user struct which contains issued key if exists
pub const USERS: Map<&Addr, User> = Map::new("USERS");

// Membership is user address, value is number of keys issued by user
pub const MEMBERSHIP_SUPPLY: Map<&Addr, Uint128> = Map::new("MEMBERSHIP_SUPPLY");

/// Note: we cannot use Map<Addr, Map<Addr, Uint128>> as map of map is not supported in cosmwasm
/// Composite key is the workaround
///
/// ALL_MEMBERSHIPS_MEMBERS and ALL_USERS_MEMBERSHIPS store the same data
/// We store it twice just to make querying easier (either get all holders of 1 key or all keys held by 1 user)

// Membership is (key issuer address, key holder address), value is amount of issuer's keys held by user
pub const ALL_MEMBERSHIPS_MEMBERS: Map<(&Addr, &Addr), Uint128> =
    Map::new("ALL_MEMBERSHIPS_MEMBERS");

// Membership is (key holder address, key issuer address), value is amount of issuer's keys held by user
pub const ALL_USERS_MEMBERSHIPS: Map<(&Addr, &Addr), Uint128> = Map::new("ALL_USERS_MEMBERSHIPS");
