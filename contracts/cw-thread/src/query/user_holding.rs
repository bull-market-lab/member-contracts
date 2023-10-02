use cosmwasm_std::{Deps, Order, StdResult};
use cw_storage_plus::{Bound, PrefixBound};

use crate::state::{ALL_USERS_HOLDINGS, DEFAULT_QUERY_LIMIT, MAX_QUERY_LIMIT};

use thread::{
    msg::{QueryUserHoldingsMsg, UserHoldingsResponse},
    user_holding::UserHolding,
};

pub fn query_user_holdings(
    deps: Deps,
    data: QueryUserHoldingsMsg,
) -> StdResult<UserHoldingsResponse> {
    let user_addr_ref = &deps.api.addr_validate(data.user_addr.as_str()).unwrap();

    let total_count = ALL_USERS_HOLDINGS
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

    let user_holdings = (match data.start_after_key_issuer_addr {
        Some(start_after_key_issuer_addr) => ALL_USERS_HOLDINGS.range(
            deps.storage,
            Some(Bound::exclusive((
                user_addr_ref,
                &deps
                    .api
                    .addr_validate(start_after_key_issuer_addr.as_str())
                    .unwrap(),
            ))),
            None,
            Order::Ascending,
        ),
        None => ALL_USERS_HOLDINGS.prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(user_addr_ref)),
            Some(PrefixBound::inclusive(user_addr_ref)),
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
        count: user_holdings.len(),
        user_holdings,
        total_count,
    })
}
