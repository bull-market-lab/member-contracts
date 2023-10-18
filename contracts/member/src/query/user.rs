use cosmwasm_std::{Deps, Order, StdResult, Uint64};
use cw_storage_plus::Bound;

use member_pkg::{
    msg::{
        QueryUserByAddrMsg, QueryUserByIDMsg, QueryUsersPaginatedByAddrMsg,
        QueryUsersPaginatedByIDMsg, UserResponse, UsersResponse,
    },
    user::User,
};

use crate::state::{ALL_USERS, DEFAULT_QUERY_LIMIT, MAX_QUERY_LIMIT, NEXT_USER_ID};

pub fn query_user_by_addr(deps: Deps, data: QueryUserByAddrMsg) -> StdResult<UserResponse> {
    let user = ALL_USERS().load(
        deps.storage,
        &deps.api.addr_validate(data.user_addr.as_str())?,
    )?;
    Ok(UserResponse { user })
}

pub fn query_user_by_id(deps: Deps, data: QueryUserByIDMsg) -> StdResult<UserResponse> {
    let user = ALL_USERS()
        .idx
        .id
        .item(deps.storage, data.user_id.u64())?
        .unwrap()
        .1;
    Ok(UserResponse { user })
}

pub fn query_users_paginated_by_addr(
    deps: Deps,
    data: QueryUsersPaginatedByAddrMsg,
) -> StdResult<UsersResponse> {
    let users = match data.start_after_user_addr {
        Some(start_after_user_addr) => {
            if data.include_start_after.unwrap_or(false) {
                ALL_USERS().range(
                    deps.storage,
                    Some(Bound::inclusive(
                        &deps.api.addr_validate(start_after_user_addr.as_str())?,
                    )),
                    None,
                    Order::Ascending,
                )
            } else {
                ALL_USERS().range(
                    deps.storage,
                    Some(Bound::exclusive(
                        &deps.api.addr_validate(start_after_user_addr.as_str())?,
                    )),
                    None,
                    Order::Ascending,
                )
            }
        }
        None => ALL_USERS().range(deps.storage, None, None, Order::Ascending),
    }
    .take(
        data.limit
            .unwrap_or(DEFAULT_QUERY_LIMIT)
            .min(MAX_QUERY_LIMIT) as usize,
    )
    .map(|item| {
        let (_, user) = item?;
        Ok(user)
    })
    .collect::<StdResult<Vec<User>>>()?;

    Ok(UsersResponse {
        total_count: (NEXT_USER_ID.load(deps.storage)? - Uint64::one()).u64() as usize,
        count: users.len(),
        users,
    })
}

pub fn query_users_paginated_by_id(
    deps: Deps,
    data: QueryUsersPaginatedByIDMsg,
) -> StdResult<UsersResponse> {
    let users = match data.start_after_user_id {
        Some(start_after_user_id) => {
            if data.include_start_after.unwrap_or(false) {
                ALL_USERS().idx.id.range(
                    deps.storage,
                    Some(Bound::inclusive(start_after_user_id.u64())),
                    None,
                    Order::Ascending,
                )
            } else {
                ALL_USERS().idx.id.range(
                    deps.storage,
                    Some(Bound::exclusive(start_after_user_id.u64())),
                    None,
                    Order::Ascending,
                )
            }
        }
        None => ALL_USERS()
            .idx
            .id
            .range(deps.storage, None, None, Order::Ascending),
    }
    .take(
        data.limit
            .unwrap_or(DEFAULT_QUERY_LIMIT)
            .min(MAX_QUERY_LIMIT) as usize,
    )
    .map(|item| {
        let (_, user) = item?;
        Ok(user)
    })
    .collect::<StdResult<Vec<User>>>()?;

    Ok(UsersResponse {
        total_count: (NEXT_USER_ID.load(deps.storage)? - Uint64::one()).u64() as usize,
        count: users.len(),
        users,
    })
}
