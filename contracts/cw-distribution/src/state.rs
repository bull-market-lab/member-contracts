use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};

use distribution::config::Config;

pub const DEFAULT_QUERY_LIMIT: u32 = 5;
pub const MAX_QUERY_LIMIT: u32 = 25;

pub const CONFIG: Item<Config> = Item::new("CONFIG");

/// Tracks global index for rewards.
/// Global index is simply a decimal number representing the amount of currency rewards paid
/// for a unit of user weight, since the beginning of time.
/// Key is membership issuer's user ID, value is global index.
pub const GLOBAL_INDICES: Map<u64, Uint128> = Map::new("GLOBAL_INDICES");

/// Key is (membership issuer's user ID, member's user ID), value is (user index, pending reward).
pub const ALL_USERS_DISTRIBUTIONS: Map<(u64, u64), (Uint128, Uint128)> =
    Map::new("ALL_USERS_DISTRIBUTIONS");
