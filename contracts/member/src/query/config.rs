use cosmwasm_std::{Deps, StdResult};

use crate::state::CONFIG;

use member_pkg::msg::ConfigResponse;

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse { config })
}
