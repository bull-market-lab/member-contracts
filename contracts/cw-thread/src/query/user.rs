use cosmwasm_std::{Deps, StdResult};

use thread::msg::{QueryUserMsg, UserResponse};

use crate::state::USERS;

pub fn query_user(deps: Deps, data: QueryUserMsg) -> StdResult<UserResponse> {
    let user = USERS.load(
        deps.storage,
        &deps.api.addr_validate(data.user_addr.as_str())?,
    )?;
    Ok(UserResponse { user })
}
