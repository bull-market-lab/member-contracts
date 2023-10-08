use cosmwasm_std::{Deps, Order, StdResult, Uint64};

use cw_storage_plus::Bound;
use membership::{
    msg::{QueryUserMsg, QueryUsersMsg, UserResponse, UsersResponse},
    user::User,
};

use crate::state::{DEFAULT_QUERY_LIMIT, MAX_QUERY_LIMIT, NEXT_USER_ID, USERS};

pub fn query_user(deps: Deps, data: QueryUserMsg) -> StdResult<UserResponse> {
    let user = USERS().load(
        deps.storage,
        &deps.api.addr_validate(data.user_addr.as_str())?,
    )?;
    Ok(UserResponse { user })
}

pub fn query_users(deps: Deps, data: QueryUsersMsg) -> StdResult<UsersResponse> {
    let users = match data.start_after_user_addr {
        Some(start_after_user_addr) => USERS().range(
            deps.storage,
            Some(Bound::exclusive(
                &deps.api.addr_validate(start_after_user_addr.as_str())?,
            )),
            None,
            Order::Ascending,
        ),
        None => USERS().range(deps.storage, None, None, Order::Ascending),
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
