use cosmwasm_std::{Deps, Order, StdResult, Uint128};

use cw_storage_plus::{Bound, PrefixBound};
use membership::{
    member::Member,
    membership::Membership,
    msg::{
        MemberCountResponse, MembersResponse, MembershipSupplyResponse, MembershipsResponse,
        QueryMemberCountMsg, QueryMembersMsg, QueryMembershipSupplyMsg, QueryMembershipsMsg,
    },
};

use crate::state::{
    ALL_MEMBERSHIPS_MEMBERS, ALL_USERS_MEMBERSHIPS, DEFAULT_QUERY_LIMIT, MAX_QUERY_LIMIT,
    MEMBERSHIP_SUPPLY,
};

pub fn query_membership_supply(
    deps: Deps,
    data: QueryMembershipSupplyMsg,
) -> StdResult<MembershipSupplyResponse> {
    let membership_issuer_addr_ref = &deps
        .api
        .addr_validate(data.membership_issuer_addr.as_str())
        .unwrap();

    let supply = MEMBERSHIP_SUPPLY
        .load(deps.storage, membership_issuer_addr_ref)
        .unwrap();

    Ok(MembershipSupplyResponse { supply })
}

pub fn query_member_count(deps: Deps, data: QueryMemberCountMsg) -> StdResult<MemberCountResponse> {
    let membership_issuer_addr_ref = &deps
        .api
        .addr_validate(data.membership_issuer_addr.as_str())
        .unwrap();

    // TODO: count is O(1), right?!?
    let count = Uint128::from(
        ALL_MEMBERSHIPS_MEMBERS
            .prefix_range(
                deps.storage,
                Some(PrefixBound::inclusive(membership_issuer_addr_ref)),
                Some(PrefixBound::inclusive(membership_issuer_addr_ref)),
                Order::Ascending,
            )
            .count() as u128,
    );

    Ok(MemberCountResponse { count })
}

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

    let memberships = (match data.start_after_membership_issuer_addr {
        Some(start_after_membership_issuer_addr) => ALL_USERS_MEMBERSHIPS.range(
            deps.storage,
            Some(Bound::exclusive((
                user_addr_ref,
                &deps
                    .api
                    .addr_validate(start_after_membership_issuer_addr.as_str())
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

pub fn query_members(deps: Deps, data: QueryMembersMsg) -> StdResult<MembersResponse> {
    let membership_issuer_addr_ref = &deps
        .api
        .addr_validate(data.membership_issuer_addr.as_str())
        .unwrap();

    let total_count = ALL_MEMBERSHIPS_MEMBERS
        .prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(membership_issuer_addr_ref)),
            Some(PrefixBound::inclusive(membership_issuer_addr_ref)),
            Order::Ascending,
        )
        .count();

    let limit = data
        .limit
        .unwrap_or(DEFAULT_QUERY_LIMIT)
        .min(MAX_QUERY_LIMIT) as usize;

    let members = (match data.start_after_member_addr {
        Some(start_after_user_addr) => ALL_MEMBERSHIPS_MEMBERS.range(
            deps.storage,
            Some(Bound::exclusive((
                membership_issuer_addr_ref,
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
            Some(PrefixBound::inclusive(membership_issuer_addr_ref)),
            Some(PrefixBound::inclusive(membership_issuer_addr_ref)),
            Order::Ascending,
        ),
    })
    .take(limit)
    .map(|item| {
        item.map(|((_, holder_addr), amount)| Member {
            holder_addr,
            amount,
        })
    })
    .collect::<StdResult<Vec<Member>>>()?;

    Ok(MembersResponse {
        count: members.len(),
        members,
        total_count,
    })
}
