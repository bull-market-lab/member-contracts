use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use friend::{config::Config, key_holder::KeyHolder, user::User, user_holding::UserHolding};

pub const DEFAULT_QUERY_LIMIT: u32 = 50;
pub const DEFAULT_QUERY_OFFSET: u32 = 0;

pub const CONFIG: Item<Config> = Item::new("config");

// Key is user address, value is user struct which contains issued key if exists
pub const USERS: Map<Addr, User> = Map::new("USERS");

// Key is key's issuer address, value is all its current holders
pub const ALL_KEYS_HOLDERS: Map<Addr, Vec<KeyHolder>> = Map::new("ALL_KEYS_HOLDERS");

// Key is user address, value is all its current holdings
pub const ALL_USERS_HOLDINGS: Map<Addr, Vec<UserHolding>> = Map::new("ALL_USERS_HOLDINGS");
