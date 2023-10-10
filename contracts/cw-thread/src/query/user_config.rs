use cosmwasm_std::{Deps, StdResult};

use thread::msg::{QueryUserConfigMsg, UserConfigResponse};

use crate::state::ALL_USER_CONFIGS;

pub fn query_user_config(deps: Deps, data: QueryUserConfigMsg) -> StdResult<UserConfigResponse> {
    let user_config = ALL_USER_CONFIGS.load(deps.storage, data.user_id.u64())?;
    Ok(UserConfigResponse { user_config })
}
