use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use distribution_pkg::config::Config;
use distribution_pkg::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use member_pkg::member_contract_querier::query_member_contract_config;

use crate::state::CONFIG;
use crate::{execute, query, ContractError};

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(
        deps.storage,
        format!("crates.io:{CONTRACT_NAME}"),
        CONTRACT_VERSION,
    )?;

    let member_contract_addr = deps.api.addr_validate(&msg.member_contract_addr)?;
    // TODO: P0: check all contract, do we need to set contract version?
    let config = Config {
        enabled: false,
        // Default to sender
        admin_addr: deps
            .api
            .addr_validate(&msg.admin_addr.unwrap_or(info.sender.to_string()))?,
        member_contract_addr: member_contract_addr.clone(),
        distribute_caller_allowlist: vec![member_contract_addr],
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let member_contract_addr = config.member_contract_addr;
    let membership_contract_config =
        query_member_contract_config(deps.as_ref(), member_contract_addr.clone());
    // TODO: P2: update this when we support user setting their own fee denom
    let fee_denom = membership_contract_config
        .default_fee_config
        .fee_denom
        .as_str();
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
        ExecuteMsg::AddToDistributeCallerAllowlist(data) => {
            cw_utils::nonpayable(&info)?;
            execute::config::add_to_distribute_caller_allowlist(deps, info, data)
        }
        ExecuteMsg::RemoveFromDistributeCallerAllowlist(data) => {
            cw_utils::nonpayable(&info)?;
            execute::config::remove_from_distribute_caller_allowlist(deps, info, data)
        }
        // TODO: P0: fix me, pass everything from membership contract
        // Do not query it inside execute as it contains un committed state
        ExecuteMsg::SetupDistributionForNewMembership(data) => {
            cw_utils::nonpayable(&info)?;
            execute::reward::setup_distribution_for_new_membership(
                deps,
                info,
                data,
                member_contract_addr,
            )
        }
        // TODO: P0: fix me, pass everything from membership contract
        // Do not query it inside execute as it contains un committed state
        ExecuteMsg::SetupDistributionForNewMember(data) => {
            cw_utils::nonpayable(&info)?;
            execute::reward::setup_distribution_for_new_member(
                deps,
                info,
                data,
                member_contract_addr,
            )
        }
        // TODO: P0: fix me, pass everything from membership contract
        // Do not query it inside execute as it contains un committed state
        ExecuteMsg::Distribute(data) => {
            cw_utils::must_pay(&info, fee_denom)?;
            execute::reward::distribute(deps, info, data, config.distribute_caller_allowlist)
        }
        // TODO: P0: fix me, pass everything from membership contract
        // Do not query it inside execute as it contains un committed state
        ExecuteMsg::UpdateUserPendingReward(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::update_user_pending_reward(deps, info, data, member_contract_addr)
        }
        ExecuteMsg::ClaimReward(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::claim_reward(deps, data, member_contract_addr, fee_denom)
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
            config.member_contract_addr,
        )?),
    }
}
