use cosmwasm_std::{Deps, Order, StdResult};
use cw_storage_plus::{Bound, PrefixBound};

use crate::state::{ALL_USERS_HOLDINGS, DEFAULT_QUERY_LIMIT, MAX_QUERY_LIMIT};

use friend::{
    msg::{QueryUserHoldingsMsg, UserHoldingsResponse},
    user_holding::UserHolding,
};

pub fn query_user_holdings(
    deps: Deps,
    data: QueryUserHoldingsMsg,
) -> StdResult<UserHoldingsResponse> {
    let total_count = ALL_USERS_HOLDINGS
        .prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(&data.user_addr)),
            Some(PrefixBound::inclusive(&data.user_addr)),
            Order::Ascending,
        )
        .count();

    let limit = data
        .limit
        .unwrap_or(DEFAULT_QUERY_LIMIT)
        .min(MAX_QUERY_LIMIT) as usize;

    let user_holdings = (match data.start_after_key_issuer_addr {
        Some(start_after_key_issuer_addr) => ALL_USERS_HOLDINGS.range(
            deps.storage,
            Some(Bound::exclusive((
                &data.user_addr,
                &start_after_key_issuer_addr,
            ))),
            None,
            Order::Ascending,
        ),
        None => ALL_USERS_HOLDINGS.prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(&data.user_addr)),
            Some(PrefixBound::inclusive(&data.user_addr)),
            Order::Ascending,
        ),
    })
    .take(limit)
    .map(|item| {
        item.map(|(k, v)| UserHolding {
            issuer_addr: k.1,
            amount: v,
        })
    })
    .collect::<StdResult<Vec<UserHolding>>>()?;

    Ok(UserHoldingsResponse {
        user_holdings,
        total_count,
    })
}
