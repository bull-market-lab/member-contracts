use cosmwasm_std::{Deps, Order, StdResult, Uint64};

use cw_storage_plus::Bound;
use distribution::msg::{QueryUserRewardMsg, UserRewardResponse};

use crate::state::{DEFAULT_QUERY_LIMIT, MAX_QUERY_LIMIT};

pub fn query_user_reward(deps: Deps, data: QueryUserRewardMsg) -> StdResult<UserRewardResponse> {
    let user = USERS().load(
        deps.storage,
        &deps.api.addr_validate(data.user_addr.as_str())?,
    )?;
    Ok(UserResponse { user })
}
