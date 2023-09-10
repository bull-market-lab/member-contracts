use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use friend::config::Config;
use friend::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::state::CONFIG;
use crate::{execute, query, ContractError};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        admin: deps
            .api
            .addr_validate(&msg.admin.unwrap_or_else(|| info.sender.to_string()))?,
        key_register_admin: deps.api.addr_validate(
            &msg.key_register_admin
                .unwrap_or_else(|| info.sender.to_string()),
        )?,
        fee_collector: deps
            .api
            .addr_validate(&msg.fee_collector.unwrap_or_else(|| info.sender.to_string()))?,
        protocol_fee_percentage: msg.protocol_fee_percentage,
        key_issuer_fee_percentage: msg.key_issuer_fee_percentage,
    };

    if config.protocol_fee_percentage.u64() > 100 {
        return Err(ContractError::ProtocolFeeTooHigh {});
    }
    if config.key_issuer_fee_percentage.u64() > 100 {
        return Err(ContractError::KeyIssuerFeeTooHigh {});
    }

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
    match msg {
        ExecuteMsg::UpdateConfig(data) => execute::config::update_config(deps, info, data),
        ExecuteMsg::Register(_data) => execute::user::register(deps, info),
        ExecuteMsg::RegisterSocialMediaAndKey(data) => {
            execute::user::register_social_media_and_key(deps, info, data)
        }
        ExecuteMsg::BuyKey(data) => execute::key::buy_key(deps, env, info, data),
        ExecuteMsg::SellKey(data) => execute::key::sell_key(deps, env, info, data),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryConfig(_data) => to_binary(&query::config::query_config(deps)?),
        QueryMsg::QueryUser(data) => to_binary(&query::user::query_user(deps, data)?),
        QueryMsg::QueryKeyHolders(data) => {
            to_binary(&query::key_holder::query_key_holders(deps, data)?)
        }
        QueryMsg::QueryUserHoldings(data) => {
            to_binary(&query::user_holding::query_user_holdings(deps, data)?)
        }
        QueryMsg::QueryKeySupply(data) => to_binary(&query::key::query_key_supply(deps, data)?),
        QueryMsg::QuerySimulateBuyKey(data) => {
            to_binary(&query::key::query_simulate_buy_key(deps, data)?)
        }
        QueryMsg::QuerySimulateSellKey(data) => {
            to_binary(&query::key::query_simulate_sell_key(deps, data)?)
        }
    }
}
