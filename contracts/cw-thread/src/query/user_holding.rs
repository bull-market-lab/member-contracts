use cosmwasm_std::{Deps, Order, StdResult};
use cw_storage_plus::{Bound, PrefixBound};

use crate::state::{ALL_USERS_MEMBERSHIPS, DEFAULT_QUERY_LIMIT, MAX_QUERY_LIMIT};

use thread::{
    msg::{MembershipsResponse, QueryMembershipsMsg},
    user_holding::Membership,
};

pub fn query_memberships(deps: Deps, data: QueryMembershipsMsg) -> StdResult<MembershipsResponse> {
    let user_addr_ref = &deps.api.addr_validate(data.user_addr.as_str()).unwrap();

    let total_count = ALL_USERS_MEMBERSHIPS
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

    let memberships = (match data.start_after_key_issuer_addr {
        Some(start_after_key_issuer_addr) => ALL_USERS_MEMBERSHIPS.range(
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
        None => ALL_USERS_MEMBERSHIPS.prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(user_addr_ref)),
            Some(PrefixBound::inclusive(user_addr_ref)),
            Order::Ascending,
        ),
    })
    .take(limit)
    .map(|item| {
        item.map(|((_, issuer_addr), amount)| Membership {
            issuer_addr,
            amount,
        })
    })
    .collect::<StdResult<Vec<Membership>>>()?;

    Ok(MembershipsResponse {
        count: memberships.len(),
        memberships,
        total_count,
    })
}
