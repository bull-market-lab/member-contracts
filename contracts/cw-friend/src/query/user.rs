use cosmwasm_std::{Deps, StdResult};

use friend::msg::{QueryUserMsg, UserResponse};

use crate::state::USERS;

pub fn query_user(deps: Deps, data: QueryUserMsg) -> StdResult<UserResponse> {
    let user = USERS.load(deps.storage, data.user_addr)?;
    Ok(UserResponse { user })
}
