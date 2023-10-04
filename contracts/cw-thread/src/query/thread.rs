use cosmwasm_std::{Deps, Order, StdResult, Uint128, Uint64};

use cw_storage_plus::{Bound, PrefixBound};
use thread::msg::{
    CostToAskResponse, CostToReplyResponse, CostToStartNewThreadResponse,
    IDsOfAllThreadMsgsInThreadResponse, IDsOfAllThreadsUserBelongToResponse,
    IDsOfAllThreadsUserCreatedResponse, QueryCostToAskMsg, QueryCostToReplyMsg,
    QueryCostToStartNewThreadMsg, QueryIDsOfAllThreadMsgsInThreadMsg,
    QueryIDsOfAllThreadsUserBelongToMsg, QueryIDsOfAllThreadsUserCreatedMsg,
    QueryThreadMsgsByIDsMsg, QueryThreadsByIDsMsg, ThreadMsgsResponse, ThreadsResponse,
};

use crate::{
    state::{
        ALL_THREADS, ALL_THREADS_MSGS, ALL_THREADS_USERS_BELONG_TO, ALL_THREADS_USERS_CREATED,
        CONFIG, DEFAULT_QUERY_LIMIT, KEY_SUPPLY, MAX_QUERY_LIMIT,
    },
    util::price::{
        calculate_price, lookup_ask_fee_percentage_of_key, lookup_reply_fee_percentage_of_key,
        lookup_thread_fee_share_config, multiply_percentage,
    },
};

pub fn query_cost_to_start_new_thread(
    deps: Deps,
    _: QueryCostToStartNewThreadMsg,
) -> StdResult<CostToStartNewThreadResponse> {
    // TODO: P0: benchmark length
    let fixed_protocol_fee = CONFIG
        .load(deps.storage)
        .unwrap()
        .protocol_fee_config
        .start_new_thread_fixed_cost;

    Ok(CostToStartNewThreadResponse {
        protocol_fee: fixed_protocol_fee,
    })
}

pub fn query_cost_to_ask(deps: Deps, data: QueryCostToAskMsg) -> StdResult<CostToAskResponse> {
    let key_issuer_addr_ref = &deps.api.addr_validate(data.ask_to_addr.as_str()).unwrap();

    let supply = KEY_SUPPLY.load(deps.storage, key_issuer_addr_ref).unwrap();

    let price_for_single_key = calculate_price(supply, Uint128::one());

    // TODO: P0: store multiply per character to config
    // TODO: P0: revise the formula
    // let price = price_for_single_key * key.thread_fee_config.ask_fee_of_key_price_percentage
    //     / Uint128::from(100 as u128)
    //     * data.content_len
    //     / Uint128::from(50 as u128);

    let fee = multiply_percentage(
        price_for_single_key,
        lookup_ask_fee_percentage_of_key(deps, key_issuer_addr_ref),
    );

    let key_trading_fee_share_config = lookup_thread_fee_share_config(deps, key_issuer_addr_ref);
    let key_issuer_fee =
        multiply_percentage(fee, key_trading_fee_share_config.key_issuer_fee_percentage);
    let key_holder_fee =
        multiply_percentage(fee, key_trading_fee_share_config.key_holder_fee_percentage);

    let protocol_fee_percentage = CONFIG
        .load(deps.storage)
        .unwrap()
        .protocol_fee_config
        .ask_in_thread_fee_percentage;
    let protocol_fee = multiply_percentage(fee, protocol_fee_percentage);

    let total_needed_from_user = protocol_fee + key_issuer_fee + key_holder_fee;

    Ok(CostToAskResponse {
        protocol_fee,
        key_issuer_fee,
        key_holder_fee,
        total_needed_from_user,
    })
}

pub fn query_cost_to_reply(
    deps: Deps,
    data: QueryCostToReplyMsg,
) -> StdResult<CostToReplyResponse> {
    let key_issuer_addr_ref = &deps.api.addr_validate(data.reply_to_addr.as_str()).unwrap();

    let supply = KEY_SUPPLY.load(deps.storage, key_issuer_addr_ref).unwrap();

    let price_for_single_key = calculate_price(supply, Uint128::one());

    // TODO: P0: store multiply per character to config
    // TODO: P0: revise the formula
    // let price = price_for_single_key * key.thread_fee_config.reply_fee_of_key_price_percentage
    //     / Uint128::from(100 as u128)
    //     * data.content_len
    //     / Uint128::from(50 as u128);

    let fee = multiply_percentage(
        price_for_single_key,
        lookup_reply_fee_percentage_of_key(deps, key_issuer_addr_ref),
    );

    let key_trading_fee_share_config = lookup_thread_fee_share_config(deps, key_issuer_addr_ref);
    let key_issuer_fee =
        multiply_percentage(fee, key_trading_fee_share_config.key_issuer_fee_percentage);
    let key_holder_fee =
        multiply_percentage(fee, key_trading_fee_share_config.key_holder_fee_percentage);

    let protocol_fee_percentage = CONFIG
        .load(deps.storage)
        .unwrap()
        .protocol_fee_config
        .reply_in_thread_fee_percentage;
    let protocol_fee = multiply_percentage(fee, protocol_fee_percentage);

    let total_needed_from_user = protocol_fee + key_issuer_fee + key_holder_fee;

    Ok(CostToReplyResponse {
        protocol_fee,
        key_issuer_fee,
        key_holder_fee,
        total_needed_from_user,
    })
}

pub fn query_ids_of_all_threads_user_belong_to(
    deps: Deps,
    data: QueryIDsOfAllThreadsUserBelongToMsg,
) -> StdResult<IDsOfAllThreadsUserBelongToResponse> {
    let user_addr_ref = &deps.api.addr_validate(data.user_addr.as_str()).unwrap();

    let total_count = ALL_THREADS_USERS_BELONG_TO
        .prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(user_addr_ref)),
            Some(PrefixBound::inclusive(user_addr_ref)),
            Order::Ascending,
        )
        .count();

    let limit = data
        .limit
        .unwrap_or(DEFAULT_QUERY_LIMIT)
        .min(MAX_QUERY_LIMIT) as usize;

    let ids_of_threads_user_belong_to: Vec<Uint64> = (match data.start_after_thread_id {
        Some(start_after_thread_id) => ALL_THREADS_USERS_BELONG_TO.range(
            deps.storage,
            Some(Bound::exclusive((
                user_addr_ref,
                start_after_thread_id.u64(),
            ))),
            None,
            Order::Ascending,
        ),
        None => ALL_THREADS_USERS_BELONG_TO.prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(user_addr_ref)),
            Some(PrefixBound::inclusive(user_addr_ref)),
            Order::Ascending,
        ),
    })
    .take(limit)
    .map(|item| item.map(|((_, thread_id), _)| Uint64::from(thread_id)))
    .collect::<StdResult<Vec<Uint64>>>()?;

    Ok(IDsOfAllThreadsUserBelongToResponse {
        count: ids_of_threads_user_belong_to.len(),
        thread_ids: ids_of_threads_user_belong_to,
        total_count,
    })
}

pub fn query_ids_of_all_threads_user_created(
    deps: Deps,
    data: QueryIDsOfAllThreadsUserCreatedMsg,
) -> StdResult<IDsOfAllThreadsUserCreatedResponse> {
    let user_addr_ref = &deps.api.addr_validate(data.user_addr.as_str()).unwrap();

    let total_count = ALL_THREADS_USERS_CREATED
        .prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(user_addr_ref)),
            Some(PrefixBound::inclusive(user_addr_ref)),
            Order::Ascending,
        )
        .count();

    let limit = data
        .limit
        .unwrap_or(DEFAULT_QUERY_LIMIT)
        .min(MAX_QUERY_LIMIT) as usize;

    let ids_of_threads_user_belong_to: Vec<Uint64> = (match data.start_after_thread_id {
        Some(start_after_thread_id) => ALL_THREADS_USERS_CREATED.range(
            deps.storage,
            Some(Bound::exclusive((
                user_addr_ref,
                start_after_thread_id.u64(),
            ))),
            None,
            Order::Ascending,
        ),
        None => ALL_THREADS_USERS_CREATED.prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(user_addr_ref)),
            Some(PrefixBound::inclusive(user_addr_ref)),
            Order::Ascending,
        ),
    })
    .take(limit)
    .map(|item| item.map(|((_, thread_id), _)| Uint64::from(thread_id)))
    .collect::<StdResult<Vec<Uint64>>>()?;

    Ok(IDsOfAllThreadsUserCreatedResponse {
        count: ids_of_threads_user_belong_to.len(),
        thread_ids: ids_of_threads_user_belong_to,
        total_count,
    })
}

pub fn query_ids_of_all_thread_msgs_in_thread(
    deps: Deps,
    data: QueryIDsOfAllThreadMsgsInThreadMsg,
) -> StdResult<IDsOfAllThreadMsgsInThreadResponse> {
    let total_count = ALL_THREADS_MSGS
        .prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(data.thread_id.u64())),
            Some(PrefixBound::inclusive(data.thread_id.u64())),
            Order::Ascending,
        )
        .count();

    let limit = data
        .limit
        .unwrap_or(DEFAULT_QUERY_LIMIT)
        .min(MAX_QUERY_LIMIT) as usize;

    let ids_of_thread_msgs_in_thread: Vec<Uint64> = (match data.start_after_thread_msg_id {
        Some(start_after_thread_msg_id) => ALL_THREADS_MSGS.range(
            deps.storage,
            Some(Bound::exclusive((
                data.thread_id.u64(),
                start_after_thread_msg_id.u64(),
            ))),
            None,
            Order::Ascending,
        ),
        None => ALL_THREADS_MSGS.prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(data.thread_id.u64())),
            Some(PrefixBound::inclusive(data.thread_id.u64())),
            Order::Ascending,
        ),
    })
    .take(limit)
    .map(|item| item.map(|((_, thread_msg_id), _)| Uint64::from(thread_msg_id)))
    .collect::<StdResult<Vec<Uint64>>>()?;

    Ok(IDsOfAllThreadMsgsInThreadResponse {
        count: ids_of_thread_msgs_in_thread.len(),
        thread_msg_ids: ids_of_thread_msgs_in_thread,
        total_count,
    })
}

pub fn query_threads_by_ids(deps: Deps, data: QueryThreadsByIDsMsg) -> StdResult<ThreadsResponse> {
    let threads = data
        .thread_ids
        .iter()
        .map(|thread_id| ALL_THREADS.load(deps.storage, thread_id.u64()).unwrap())
        .collect();

    Ok(ThreadsResponse { threads })
}

pub fn query_thread_msgs_by_ids(
    deps: Deps,
    data: QueryThreadMsgsByIDsMsg,
) -> StdResult<ThreadMsgsResponse> {
    let thread_msgs = data
        .thread_and_thread_msg_ids
        .iter()
        .map(|(thread_id, thread_msg_id)| {
            ALL_THREADS_MSGS
                .load(deps.storage, (thread_id.u64(), thread_msg_id.u64()))
                .unwrap()
        })
        .collect();

    Ok(ThreadMsgsResponse { thread_msgs })
}
