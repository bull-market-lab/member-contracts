use cosmwasm_std::{Deps, StdResult};

use crate::state::{ALL_USERS_HOLDINGS, DEFAULT_QUERY_LIMIT, DEFAULT_QUERY_OFFSET};

use friend::{
    msg::{QueryUserHoldingsMsg, UserHoldingsResponse},
    user_holding::UserHolding,
};

pub fn query_user_holdings(
    deps: Deps,
    data: QueryUserHoldingsMsg,
) -> StdResult<UserHoldingsResponse> {
    let offset = data.offset.unwrap_or(DEFAULT_QUERY_OFFSET);
    let limit = data.limit.unwrap_or(DEFAULT_QUERY_LIMIT);
    let user_holdings = ALL_USERS_HOLDINGS
        .load(deps.storage, data.user_addr)?
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect::<Vec<UserHolding>>();

    Ok(UserHoldingsResponse { user_holdings })
}
