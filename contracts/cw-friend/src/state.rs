use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

use friend::{config::Config, user::User};

pub const DEFAULT_QUERY_LIMIT: u32 = 5;
pub const MAX_QUERY_LIMIT: u32 = 25;

pub const CONFIG: Item<Config> = Item::new("config");

// Key is user address, value is user struct which contains issued key if exists
pub const USERS: Map<&Addr, User> = Map::new("USERS");

/// Note: we cannot use Map<Addr, Map<Addr, Uint128>> as it is not supported in cosmwasm
/// Composite key is the workaround
///
/// ALL_KEYS_HOLDERS and ALL_USERS_HOLDINGS store the same data
/// We store it twice just to make querying easier (either get all holders of 1 key or all keys held by 1 user)

// Key is (key's issuer address, user address)
// Value is amount of issuer's keys held by user
pub const ALL_KEYS_HOLDERS: Map<(&Addr, &Addr), Uint128> = Map::new("ALL_KEYS_HOLDERS");

// Key is (user address, key's issuer address)
// Value is amount of issuer's keys held by user
pub const ALL_USERS_HOLDINGS: Map<(&Addr, &Addr), Uint128> = Map::new("ALL_USERS_HOLDINGS");
