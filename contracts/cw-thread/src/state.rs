use cosmwasm_std::{Addr, Uint128, Uint64};
use cw_storage_plus::{Item, Map};

use thread::{config::Config, thread::Thread, thread_msg::ThreadMsg, user::User};

pub const DEFAULT_QUERY_LIMIT: u32 = 5;
pub const MAX_QUERY_LIMIT: u32 = 25;

pub const CONFIG: Item<Config> = Item::new("CONFIG");

// Next available monotonically increasing global unique ID to identify each thread
// Start from 1
pub const NEXT_THREAD_ID: Item<Uint64> = Item::new("NEXT_THREAD_ID");
// Next available monotonically increasing ID to identify each message in this thread
// Unique per thread
// Start from 1
pub const NEXT_THREAD_MSG_ID: Map<u64, Uint64> = Map::new("NEXT_THREAD_MSG_ID");

// Key is user address, value is user struct which contains issued key if exists
pub const USERS: Map<&Addr, User> = Map::new("USERS");

// Key is user address, value is number of keys issued by user
pub const KEY_SUPPLY: Map<&Addr, Uint128> = Map::new("KEY_SUPPLY");

/// Note: we cannot use Map<Addr, Map<Addr, Uint128>> as map of map is not supported in cosmwasm
/// Composite key is the workaround
///
/// ALL_KEYS_HOLDERS and ALL_USERS_HOLDINGS store the same data
/// We store it twice just to make querying easier (either get all holders of 1 key or all keys held by 1 user)

// Key is (key's issuer address, user address), value is amount of issuer's keys held by user
pub const ALL_KEYS_HOLDERS: Map<(&Addr, &Addr), Uint128> = Map::new("ALL_KEYS_HOLDERS");

// Key is (user address, key's issuer address), value is amount of issuer's keys held by user
pub const ALL_USERS_HOLDINGS: Map<(&Addr, &Addr), Uint128> = Map::new("ALL_USERS_HOLDINGS");

/*
    requests:
    1. get all conversations that are asked to me
    2. get all conversations that I asked to others
    3. get all messages sent by me in a conversation

    some assumptions
    1. there will be a lot of conversations
    2. there will not be a crazy amount of messages in a conversation
*/

// Key is thread ID, value is thread struct
pub const ALL_THREADS: Map<u64, Thread> = Map::new("ALL_THREADS");

// Key is (user address, thread ID), value is a dummy value that is always true  (to mimic a set)
// We do not store the thread struct directly in value to save space
// As each thread will be stored multiple times (once for each participant)
pub const ALL_THREADS_USERS_BELONG_TO: Map<(&Addr, u64), bool> =
    Map::new("ALL_THREADS_USERS_BELONG_TO");

// Key is (user address, thread ID), value is a dummy value that is always true  (to mimic a set)
// We do not store the thread struct directly in value to save space
// As each thread will be stored multiple times (once for each participant)
pub const ALL_THREADS_USERS_CREATED: Map<(&Addr, u64), bool> =
    Map::new("ALL_THREADS_USERS_CREATED");

// Key is (thread ID, thread message ID), value is thread message
pub const ALL_THREADS_MSGS: Map<(u64, u64), ThreadMsg> = Map::new("ALL_THREADS_MSGS");

// Key is (thread ID, thread unanswered question message ID), value is a dummy value that is always true  (to mimic a set)
pub const ALL_THREADS_UNANSWERED_QUESTION_MSGS: Map<(u64, u64), bool> =
    Map::new("ALL_THREADS_UNANSWERED_QUESTION_MSGS");
