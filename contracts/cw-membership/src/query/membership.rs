use cosmwasm_std::{Deps, Order, StdResult, Uint64};

use cw_storage_plus::{Bound, PrefixBound};
use membership::{
    msg::{
        MemberCountResponse, MembersResponse, MembershipSupplyResponse, MembershipsResponse,
        QueryMemberCountMsg, QueryMembersMsg, QueryMembershipSupplyMsg, QueryMembershipsMsg,
    },
    user::{Member, Membership},
};

use crate::state::{
    ALL_MEMBERSHIPS_MEMBERS, ALL_USERS_MEMBERSHIPS, DEFAULT_QUERY_LIMIT, MAX_QUERY_LIMIT, USERS,
};

pub fn query_membership_supply(
    deps: Deps,
    data: QueryMembershipSupplyMsg,
) -> StdResult<MembershipSupplyResponse> {
    let supply = USERS()
        .idx
        .id
        .item(deps.storage, data.membership_issuer_user_id.u64())?
        .unwrap()
        .1
        .membership_issued_by_me
        .unwrap()
        .membership_supply;

    Ok(MembershipSupplyResponse { supply })
}

pub fn query_member_count(deps: Deps, data: QueryMemberCountMsg) -> StdResult<MemberCountResponse> {
    let count = USERS()
        .idx
        .id
        .item(deps.storage, data.membership_issuer_user_id.u64())?
        .unwrap()
        .1
        .membership_issued_by_me
        .unwrap()
        .member_count;

    Ok(MemberCountResponse { count })
}

pub fn query_memberships(deps: Deps, data: QueryMembershipsMsg) -> StdResult<MembershipsResponse> {
    let user_id = data.user_id.u64();

    let memberships = match data.start_after_membership_issuer_user_id {
        Some(start_after_membership_issuer_user_id) => ALL_USERS_MEMBERSHIPS.range(
            deps.storage,
            Some(if data.include_start_after.unwrap_or(false) {
                Bound::inclusive((user_id, start_after_membership_issuer_user_id.u64()))
            } else {
                Bound::exclusive((user_id, start_after_membership_issuer_user_id.u64()))
            }),
            None,
            Order::Ascending,
        ),
        None => ALL_USERS_MEMBERSHIPS.prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(user_id)),
            Some(PrefixBound::inclusive(user_id)),
            Order::Ascending,
        ),
    }
    .take(
        data.limit
            .unwrap_or(DEFAULT_QUERY_LIMIT)
            .min(MAX_QUERY_LIMIT) as usize,
    )
    .map(|item| {
        item.map(|((_, issuer_user_id), amount)| Membership {
            issuer_user_id: Uint64::from(issuer_user_id),
            amount,
        })
    })
    .collect::<StdResult<Vec<Membership>>>()?;

    let total_count = USERS()
        .idx
        .id
        .item(deps.storage, user_id)?
        .unwrap()
        .1
        .user_member_count
        .u128() as usize;

    Ok(MembershipsResponse {
        count: memberships.len(),
        memberships,
        total_count,
    })
}

pub fn query_members(deps: Deps, data: QueryMembersMsg) -> StdResult<MembersResponse> {
    let membership_issuer_user_id = data.membership_issuer_user_id.u64();

    let limit = data
        .limit
        .unwrap_or(DEFAULT_QUERY_LIMIT)
        .min(MAX_QUERY_LIMIT) as usize;

    let members = (match data.start_after_member_user_id {
        Some(start_after_user_id) => ALL_MEMBERSHIPS_MEMBERS.range(
            deps.storage,
            Some(if data.include_start_after.unwrap_or(false) {
                Bound::inclusive((membership_issuer_user_id, start_after_user_id.u64()))
            } else {
                Bound::exclusive((membership_issuer_user_id, start_after_user_id.u64()))
            }),
            None,
            Order::Ascending,
        ),
        None => ALL_MEMBERSHIPS_MEMBERS.prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(membership_issuer_user_id)),
            Some(PrefixBound::inclusive(membership_issuer_user_id)),
            Order::Ascending,
        ),
    })
    .take(limit)
    .map(|item| {
        item.map(|((_, member_user_id), amount)| Member {
            member_user_id: Uint64::from(member_user_id),
            amount,
        })
    })
    .collect::<StdResult<Vec<Member>>>()?;

    let total_count = USERS()
        .idx
        .id
        .item(deps.storage, membership_issuer_user_id)?
        .unwrap()
        .1
        .membership_issued_by_me
        .unwrap()
        .member_count
        .u128() as usize;

    Ok(MembersResponse {
        count: members.len(),
        members,
        total_count,
    })
}
