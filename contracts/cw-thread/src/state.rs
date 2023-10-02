use cosmwasm_std::{Addr, Uint128, Uint64};
use cw_storage_plus::{Item, Map};

use thread::{config::Config, qa_thread::QAThread, qa_thread_msg::QAThreadMsg, user::User};

pub const DEFAULT_QUERY_LIMIT: u32 = 5;
pub const MAX_QUERY_LIMIT: u32 = 25;

pub const CONFIG: Item<Config> = Item::new("CONFIG");

// Next available monotonically increasing global unique ID to identify each QA thread
// Start from 1
pub const NEXT_QA_THREAD_ID: Item<Uint64> = Item::new("NEXT_QA_THREAD_ID");
// Next available monotonically increasing ID to identify each message in this QA thread
// Unique per QA thread
// Start from 1
pub const NEXT_QA_THREAD_MSG_ID: Map<u64, Uint64> = Map::new("NEXT_QA_THREAD_MSG_ID");

// Key is user address, value is user struct which contains issued key if exists
pub const USERS: Map<&Addr, User> = Map::new("USERS");

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

// Key is QA thread ID, value is QA thread struct
pub const ALL_QA_THREADS: Map<u64, QAThread> = Map::new("ALL_QA_THREADS");

// Key is (user address, QA thread ID), value is same QA thread ID from key (to mimic a set)
// We do not store the QA thread struct directly in value to save space
// As each QA thread will be stored multiple times (once for each participant)
pub const ALL_QA_THREADS_USERS_BELONG_TO: Map<(&Addr, u64), Uint64> =
    Map::new("ALL_QA_THREADS_USERS_BELONG_TO");

// Key is (QA thread ID, QA thread message ID), value is QA thread message
pub const ALL_QA_THREADS_MSGS: Map<(u64, u64), QAThreadMsg> = Map::new("ALL_QA_THREADS_MSGS");
