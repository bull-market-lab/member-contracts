use cosmwasm_std::{Deps, Order, StdResult};
use cw_storage_plus::{Bound, PrefixBound};

use crate::state::{ALL_MEMBERSHIPS_MEMBERS, DEFAULT_QUERY_LIMIT, MAX_QUERY_LIMIT};

use thread::{
    key_holder::MembershipHolder,
    msg::{MembershipHoldersResponse, QueryMembershipHoldersMsg},
};

pub fn query_key_holders(
    deps: Deps,
    data: QueryMembershipHoldersMsg,
) -> StdResult<MembershipHoldersResponse> {
    let key_issuer_addr_ref = &deps
        .api
        .addr_validate(data.key_issuer_addr.as_str())
        .unwrap();

    let total_count = ALL_MEMBERSHIPS_MEMBERS
        .prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(key_issuer_addr_ref)),
            Some(PrefixBound::inclusive(key_issuer_addr_ref)),
            Order::Ascending,
        )
        .count();

    let limit = data
        .limit
        .unwrap_or(DEFAULT_QUERY_LIMIT)
        .min(MAX_QUERY_LIMIT) as usize;

    let key_holders: Vec<MembershipHolder> = (match data.start_after_user_addr {
        Some(start_after_user_addr) => ALL_MEMBERSHIPS_MEMBERS.range(
            deps.storage,
            Some(Bound::exclusive((
                key_issuer_addr_ref,
                &deps
                    .api
                    .addr_validate(start_after_user_addr.as_str())
                    .unwrap(),
            ))),
            None,
            Order::Ascending,
        ),
        None => ALL_MEMBERSHIPS_MEMBERS.prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(key_issuer_addr_ref)),
            Some(PrefixBound::inclusive(key_issuer_addr_ref)),
            Order::Ascending,
        ),
    })
    .take(limit)
    .map(|item| {
        item.map(|((_, holder_addr), amount)| MembershipHolder {
            holder_addr,
            amount,
        })
    })
    .collect::<StdResult<Vec<MembershipHolder>>>()?;

    Ok(MembershipHoldersResponse {
        count: key_holders.len(),
        key_holders,
        total_count,
    })
}
