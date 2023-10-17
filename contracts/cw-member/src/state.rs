use cosmwasm_std::{Addr, Uint128, Uint64};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, UniqueIndex};

use member::{config::Config, user::User};

pub const DEFAULT_QUERY_LIMIT: u32 = 5;
pub const MAX_QUERY_LIMIT: u32 = 25;

pub const CONFIG: Item<Config> = Item::new("CONFIG");

// Next available monotonically increasing global unique ID to identify each user
// Start from 1
pub const NEXT_USER_ID: Item<Uint64> = Item::new("NEXT_USER_ID");

// TODO: P0: add another index to query user by social media handle?
pub struct UserIndexes<'a> {
    pub id: UniqueIndex<'a, u64, User>,
}

impl<'a> IndexList<User> for UserIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<User>> + '_> {
        let v: Vec<&dyn Index<User>> = vec![&self.id];
        Box::new(v.into_iter())
    }
}

// TODO: P1: benchmark should we use address as key and ID as index or the other way around?
// Key is user address, indexed key is user ID, value is user struct
#[allow(non_snake_case)]
pub fn ALL_USERS<'a>() -> IndexedMap<'a, &'a Addr, User, UserIndexes<'a>> {
    let indexes = UserIndexes {
        id: UniqueIndex::new(|user| (user.id.u64()), "ALL_USERS_USER_ID"),
    };
    IndexedMap::new("ALL_USERS", indexes)
}

/// Note: we cannot use Map<Addr, Map<Addr, Uint128>> as map of map is not supported in cosmwasm
/// Composite key is the workaround
///
/// ALL_MEMBERSHIPS_MEMBERS and ALL_USERS_MEMBERSHIPS store the same data
/// We store it twice just to make querying easier (either get all holders of 1 key or all keys held by 1 user)
// TODO: P2: decide if we should store this in indexer, as we only need ALL_MEMBERSHIPS_MEMBERS or ALL_USERS_MEMBERSHIPS

// Key is (membership issuer's user ID, member's user ID), value is amount of issuer's keys held by user
pub const ALL_MEMBERSHIPS_MEMBERS: Map<(u64, u64), Uint128> = Map::new("ALL_MEMBERSHIPS_MEMBERS");

// Key is (member's user ID, membership issuer's user ID), value is amount of issuer's keys held by user
pub const ALL_USERS_MEMBERSHIPS: Map<(u64, u64), Uint128> = Map::new("ALL_USERS_MEMBERSHIPS");
