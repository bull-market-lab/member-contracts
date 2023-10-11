use cosmwasm_std::{Deps, Order, StdResult, Uint128, Uint64};

use cw_storage_plus::{Bound, PrefixBound};
use thread::{
    config::Config,
    msg::{
        CostToAskInThreadResponse, CostToReplyInThreadResponse, CostToStartNewThreadResponse,
        IDsOfAllThreadMsgsInThreadResponse, IDsOfAllThreadsUserCreatedResponse,
        IDsOfAllThreadsUserParticipatedResponse, QueryCostToAskInThreadMsg,
        QueryCostToReplyInThreadMsg, QueryIDsOfAllThreadMsgsInThreadMsg,
        QueryIDsOfAllThreadsUserCreatedMsg, QueryIDsOfAllThreadsUserParticipatedMsg,
        QueryThreadMsgsByIDsMsg, QueryThreadsByIDsMsg, ThreadMsgsResponse, ThreadsResponse,
    },
};

use crate::{
    state::{
        ALL_THREADS, ALL_THREADS_MSGS, ALL_USERS_CREATED_THREADS, ALL_USERS_PARTICIPATED_THREADS,
        ALL_USERS_THREAD_STATS, ALL_USER_CONFIGS, DEFAULT_QUERY_LIMIT, MAX_QUERY_LIMIT,
    },
    util::{
        membership::query_membership_supply,
        price::{
            calculate_price, lookup_ask_fee_percentage_of_membership,
            lookup_ask_fee_to_thread_creator_percentage_of_membership,
            lookup_fee_share_to_all_members_percentage, lookup_fee_share_to_issuer_percentage,
            lookup_reply_fee_percentage_of_membership,
            lookup_reply_fee_to_thread_creator_percentage_of_membership, multiply_percentage,
        },
    },
};

pub fn query_cost_to_start_new_thread(config: Config) -> StdResult<CostToStartNewThreadResponse> {
    // TODO: P0: benchmark length
    let fixed_protocol_fee = config.protocol_fee_start_new_thread_fixed_cost;

    Ok(CostToStartNewThreadResponse {
        protocol_fee: fixed_protocol_fee,
    })
}

pub fn query_cost_to_ask_in_thread(
    deps: Deps,
    data: QueryCostToAskInThreadMsg,
    config: Config,
) -> StdResult<CostToAskInThreadResponse> {
    let ask_to_user_id = data.ask_to_user_id.u64();
    let thread_creator_user_id = data.thread_creator_user_id.u64();
    let thread_creator_config = ALL_USER_CONFIGS
        .load(deps.storage, thread_creator_user_id)
        .unwrap();

    let supply = query_membership_supply(deps, config.membership_contract_addr, ask_to_user_id);

    let price_for_single_membership = calculate_price(supply, Uint128::one());

    let membership_issuer_fee_percentage = lookup_fee_share_to_issuer_percentage(
        config.default_share_to_issuer_percentage,
        thread_creator_config.share_to_issuer_percentage,
    );
    let all_members_percentage = lookup_fee_share_to_all_members_percentage(
        config.default_share_to_all_members_percentage,
        thread_creator_config.share_to_all_members_percentage,
    );

    let ask_fee = multiply_percentage(
        price_for_single_membership,
        lookup_ask_fee_percentage_of_membership(
            config.default_ask_fee_percentage_of_membership,
            thread_creator_config.ask_fee_percentage_of_membership,
        ),
    );

    let ask_to_membership_issuer_fee =
        multiply_percentage(ask_fee, membership_issuer_fee_percentage);
    let ask_to_membership_all_members_fee = multiply_percentage(ask_fee, all_members_percentage);

    let protocol_fee_percentage = config.protocol_fee_ask_in_thread_fee_percentage;
    let protocol_fee = multiply_percentage(ask_fee, protocol_fee_percentage);

    let thread_creator_fee = multiply_percentage(
        price_for_single_membership,
        lookup_ask_fee_to_thread_creator_percentage_of_membership(
            config.default_ask_fee_to_thread_creator_percentage_of_membership,
            thread_creator_config.ask_fee_to_thread_creator_percentage_of_membership,
        ),
    );

    // 0 if thread creator is membership issuer
    let (thread_creator_membership_issuer_fee, thread_creator_membership_all_members_fee) =
        if thread_creator_user_id == ask_to_user_id {
            (Uint128::zero(), Uint128::zero())
        } else {
            (
                multiply_percentage(thread_creator_fee, membership_issuer_fee_percentage),
                multiply_percentage(thread_creator_fee, all_members_percentage),
            )
        };

    let total_needed_from_user = protocol_fee
        + ask_to_membership_issuer_fee
        + ask_to_membership_all_members_fee
        + thread_creator_membership_issuer_fee
        + thread_creator_membership_all_members_fee;

    Ok(CostToAskInThreadResponse {
        protocol_fee,
        ask_to_membership_issuer_fee,
        ask_to_membership_all_members_fee,
        thread_creator_membership_issuer_fee,
        thread_creator_membership_all_members_fee,
        total_needed_from_user,
    })
}

pub fn query_cost_to_reply_in_thread(
    deps: Deps,
    data: QueryCostToReplyInThreadMsg,
    config: Config,
) -> StdResult<CostToReplyInThreadResponse> {
    let reply_to_user_id = data.reply_to_user_id.u64();
    let thread_creator_user_id = data.thread_creator_user_id.u64();
    let thread_creator_config = ALL_USER_CONFIGS
        .load(deps.storage, thread_creator_user_id)
        .unwrap();

    let supply = query_membership_supply(deps, config.membership_contract_addr, reply_to_user_id);

    let price_for_single_membership = calculate_price(supply, Uint128::one());

    let membership_issuer_fee_percentage = lookup_fee_share_to_issuer_percentage(
        config.default_share_to_issuer_percentage,
        thread_creator_config.share_to_issuer_percentage,
    );
    let all_members_percentage = lookup_fee_share_to_all_members_percentage(
        config.default_share_to_all_members_percentage,
        thread_creator_config.share_to_all_members_percentage,
    );

    let reply_fee = multiply_percentage(
        price_for_single_membership,
        lookup_reply_fee_percentage_of_membership(
            config.default_reply_fee_percentage_of_membership,
            thread_creator_config.reply_fee_percentage_of_membership,
        ),
    );

    let reply_to_membership_issuer_fee =
        multiply_percentage(reply_fee, membership_issuer_fee_percentage);
    let reply_to_membership_all_members_fee =
        multiply_percentage(reply_fee, all_members_percentage);

    let protocol_fee_percentage = config.protocol_fee_ask_in_thread_fee_percentage;
    let protocol_fee = multiply_percentage(reply_fee, protocol_fee_percentage);

    let thread_creator_fee = multiply_percentage(
        price_for_single_membership,
        lookup_reply_fee_to_thread_creator_percentage_of_membership(
            config.default_reply_fee_to_thread_creator_percentage_of_membership,
            thread_creator_config.reply_fee_to_thread_creator_percentage_of_membership,
        ),
    );

    // 0 if thread creator is membership issuer
    let (thread_creator_membership_issuer_fee, thread_creator_membership_all_members_fee) =
        if thread_creator_user_id == reply_to_user_id {
            (Uint128::zero(), Uint128::zero())
        } else {
            (
                multiply_percentage(thread_creator_fee, membership_issuer_fee_percentage),
                multiply_percentage(thread_creator_fee, all_members_percentage),
            )
        };

    let total_needed_from_user = protocol_fee
        + reply_to_membership_issuer_fee
        + reply_to_membership_all_members_fee
        + thread_creator_membership_issuer_fee
        + thread_creator_membership_all_members_fee;

    Ok(CostToReplyInThreadResponse {
        protocol_fee,
        reply_to_membership_issuer_fee,
        reply_to_membership_all_members_fee,
        thread_creator_membership_issuer_fee,
        thread_creator_membership_all_members_fee,
        total_needed_from_user,
    })
}

pub fn query_ids_of_all_threads_user_participated(
    deps: Deps,
    data: QueryIDsOfAllThreadsUserParticipatedMsg,
) -> StdResult<IDsOfAllThreadsUserParticipatedResponse> {
    let user_id = data.user_id.u64();

    let limit = data
        .limit
        .unwrap_or(DEFAULT_QUERY_LIMIT)
        .min(MAX_QUERY_LIMIT) as usize;

    let ids_of_threads_user_belong_to: Vec<Uint64> = (match data.start_after_thread_id {
        Some(start_after_thread_id) => ALL_USERS_PARTICIPATED_THREADS.range(
            deps.storage,
            if data.include_start_after.unwrap_or(false) {
                Some(Bound::inclusive((user_id, start_after_thread_id.u64())))
            } else {
                Some(Bound::exclusive((user_id, start_after_thread_id.u64())))
            },
            // TODO: Test all pagination, maybe we should set max to user_id as well??
            None,
            Order::Ascending,
        ),
        None => ALL_USERS_PARTICIPATED_THREADS.prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(user_id)),
            Some(PrefixBound::inclusive(user_id)),
            Order::Ascending,
        ),
    })
    .take(limit)
    .map(|item| item.map(|((_, thread_id), _)| Uint64::from(thread_id)))
    .collect::<StdResult<Vec<Uint64>>>()?;

    Ok(IDsOfAllThreadsUserParticipatedResponse {
        count: ids_of_threads_user_belong_to.len(),
        thread_ids: ids_of_threads_user_belong_to,
        total_count: ALL_USERS_THREAD_STATS.load(deps.storage, user_id)?.1.u128() as usize,
    })
}

pub fn query_ids_of_all_threads_user_created(
    deps: Deps,
    data: QueryIDsOfAllThreadsUserCreatedMsg,
) -> StdResult<IDsOfAllThreadsUserCreatedResponse> {
    let user_id = data.user_id.u64();

    let limit = data
        .limit
        .unwrap_or(DEFAULT_QUERY_LIMIT)
        .min(MAX_QUERY_LIMIT) as usize;

    let ids_of_threads_user_belong_to: Vec<Uint64> = (match data.start_after_thread_id {
        Some(start_after_thread_id) => ALL_USERS_CREATED_THREADS.range(
            deps.storage,
            if data.include_start_after.unwrap_or(false) {
                Some(Bound::inclusive((user_id, start_after_thread_id.u64())))
            } else {
                Some(Bound::exclusive((user_id, start_after_thread_id.u64())))
            },
            // TODO: Test all pagination, maybe we should set max to user_id as well??
            None,
            Order::Ascending,
        ),
        None => ALL_USERS_CREATED_THREADS.prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(user_id)),
            Some(PrefixBound::inclusive(user_id)),
            Order::Ascending,
        ),
    })
    .take(limit)
    .map(|item| item.map(|((_, thread_id), _)| Uint64::from(thread_id)))
    .collect::<StdResult<Vec<Uint64>>>()?;

    Ok(IDsOfAllThreadsUserCreatedResponse {
        count: ids_of_threads_user_belong_to.len(),
        thread_ids: ids_of_threads_user_belong_to,
        total_count: ALL_USERS_THREAD_STATS.load(deps.storage, user_id)?.0.u128() as usize,
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
