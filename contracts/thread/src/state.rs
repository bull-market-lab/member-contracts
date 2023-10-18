use cosmwasm_std::{Uint128, Uint64};
use cw_storage_plus::{Item, Map};

use thread_pkg::{
    config::Config,
    thread::{Thread, ThreadMsg},
    user_config::UserConfig,
};

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

// Key is user ID in membership contract, value is user struct which contains fee config
pub const ALL_USER_CONFIGS: Map<u64, UserConfig> = Map::new("ALL_USER_CONFIGS");

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

// Key is user ID in membership contract
// Value is (total number of threads created by user, total number of threads participated by user)
pub const ALL_USERS_THREAD_STATS: Map<u64, (Uint128, Uint128)> = Map::new("ALL_USERS_THREAD_STATS");

// Key is (user ID in membership contract, thread ID)
// Value is a dummy value to mimic a set
// We do not store the thread struct directly in value to save space
// As each thread will be stored multiple times (once for each participant)
// TODO: P2: decide should we store this onchain or in indexer
pub const ALL_USERS_CREATED_THREADS: Map<(u64, u64), bool> = Map::new("ALL_USERS_CREATED_THREADS");

// Key is (user ID in membership contract, thread ID)
// Value is a dummy value to mimic a set
// We do not store the thread struct directly in value to save space
// As each thread will be stored multiple times (once for each participant)
// TODO: P2: decide should we store this onchain or in indexer
pub const ALL_USERS_PARTICIPATED_THREADS: Map<(u64, u64), bool> =
    Map::new("ALL_USERS_PARTICIPATED_THREADS");

// Key is thread ID, value is number of messages in this thread
pub const ALL_THREADS_MSGS_COUNT: Map<u64, Uint128> = Map::new("ALL_THREADS_MSGS_COUNT");

// Key is (thread ID, thread message ID), value is thread message
pub const ALL_THREADS_MSGS: Map<(u64, u64), ThreadMsg> = Map::new("ALL_THREADS_MSGS");

// Key is (user ID in membership contract, thread ID, thread unanswered question message ID)
// Value is a dummy value that is always true  (to mimic a set)
// TODO: P2: decide should we store this onchain or in indexer
pub const ALL_USERS_UNANSWERED_QUESTIONS: Map<(u64, u64, u64), bool> =
    Map::new("ALL_USERS_UNANSWERED_QUESTIONS");
