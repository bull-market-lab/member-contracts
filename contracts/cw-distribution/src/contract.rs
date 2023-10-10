use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use distribution::config::Config;
use distribution::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::state::CONFIG;
use crate::util::membership::query_membership_contract_config;
use crate::{execute, query, ContractError};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // TODO: P0: check all contract, do we need to set contract version?
    let config = Config {
        enabled: false,
        // Default to sender
        admin_addr: deps
            .api
            .addr_validate(&msg.admin_addr.unwrap_or(info.sender.to_string()))?,
        membership_contract_addr: deps.api.addr_validate(&msg.membership_contract_addr)?,
        // TODO: P2: decide if we need minimum eligible weight
        // minimum_eligible_weight: msg.minimum_eligible_weight.unwrap_or(Uint128::zero()),
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let membership_contract_addr = config.membership_contract_addr;
    let membership_contract_config =
        query_membership_contract_config(deps.as_ref(), membership_contract_addr.clone());
    let fee_denom = membership_contract_config.fee_denom.as_str();
    match msg {
        ExecuteMsg::Enable(_) => {
            cw_utils::nonpayable(&info)?;
            execute::config::enable(deps, info)
        }
        ExecuteMsg::Disable(_) => {
            cw_utils::nonpayable(&info)?;
            execute::config::disable(deps, info)
        }
        ExecuteMsg::UpdateConfig(data) => {
            cw_utils::nonpayable(&info)?;
            execute::config::update_config(deps, info, data)
        }
        ExecuteMsg::SetupDistributionForNewMembership(data) => {
            cw_utils::nonpayable(&info)?;
            execute::reward::setup_distribution_for_new_membership(
                deps,
                info,
                data,
                membership_contract_addr,
            )
        }
        ExecuteMsg::SetupDistributionForNewMember(data) => {
            cw_utils::nonpayable(&info)?;
            execute::reward::setup_distribution_for_new_member(
                deps,
                info,
                data,
                membership_contract_addr,
            )
        }
        ExecuteMsg::Distribute(data) => {
            cw_utils::must_pay(&info, fee_denom)?;
            execute::reward::distribute(deps, info, data, membership_contract_addr)
        }
        ExecuteMsg::UpdateUserPendingReward(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::update_user_pending_reward(deps, info, data, membership_contract_addr)
        }
        ExecuteMsg::ClaimReward(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::claim_reward(deps, env, data, membership_contract_addr, fee_denom)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    match msg {
        QueryMsg::QueryConfig(_) => to_binary(&query::config::query_config(deps)?),
        QueryMsg::QueryUserReward(data) => to_binary(&query::user::query_user_reward(
            deps,
            data,
            config.membership_contract_addr,
        )?),
    }
}
