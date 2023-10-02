use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint64,
};

use thread::config::Config;
use thread::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::state::{CONFIG, NEXT_THREAD_ID};
use crate::{execute, query, ContractError};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        admin_addr: deps
            .api
            .addr_validate(&msg.admin_addr.unwrap_or(info.sender.to_string()))?,
        registration_admin_addr: deps.api.addr_validate(
            &msg.registration_admin_addr
                .unwrap_or(info.sender.to_string()),
        )?,
        protocol_fee_collector_addr: deps.api.addr_validate(
            &msg.protocol_fee_collector_addr
                .unwrap_or(info.sender.to_string()),
        )?,
        // TODO: validate denom, and use noble USDC
        fee_denom: msg.fee_denom.unwrap_or("uluna".to_string()),
        max_thread_title_length: msg
            .max_thread_title_length
            .unwrap_or(Uint64::from(50 as u64)),
        max_thread_msg_length: msg
            .max_thread_msg_length
            .unwrap_or(Uint64::from(500 as u64)),
    };

    CONFIG.save(deps.storage, &config)?;

    NEXT_THREAD_ID.save(deps.storage, &Uint64::one())?;

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
    match msg {
        ExecuteMsg::UpdateConfig(data) => {
            cw_utils::nonpayable(&info)?;
            execute::config::update_config(deps, info, data)
        }
        ExecuteMsg::Register() => {
            cw_utils::nonpayable(&info)?;
            execute::user::register(deps, info)
        }
        ExecuteMsg::LinkSocialMedia(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::link_social_media(deps, info, data, config)
        }
        ExecuteMsg::RegisterKey(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::register_key(deps, info, data, config)
        }
        ExecuteMsg::UpdateTradingFeeConfig(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::update_trading_fee_config(deps, info, data)
        }
        ExecuteMsg::UpdateThreadFeeConfig(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::update_thread_fee_config(deps, info, data)
        }
        ExecuteMsg::BuyKey(data) => {
            let user_paid_amount = cw_utils::must_pay(&info, config.fee_denom.as_str())?;
            execute::key::buy_key(deps, env, info, data, config, user_paid_amount)
        }
        ExecuteMsg::SellKey(data) => {
            let user_paid_amount = cw_utils::must_pay(&info, config.fee_denom.as_str())?;
            execute::key::sell_key(deps, env, info, data, config, user_paid_amount)
        }
        ExecuteMsg::Ask(data) => {
            let user_paid_amount = cw_utils::must_pay(&info, config.fee_denom.as_str())?;
            execute::thread::ask(deps, env, info, data, config, user_paid_amount)
        }
        ExecuteMsg::Answer(data) => {
            cw_utils::nonpayable(&info)?;
            execute::thread::answer(deps, info, data, config)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryConfig(_) => to_binary(&query::config::query_config(deps)?),
        QueryMsg::QueryUser(data) => to_binary(&query::user::query_user(deps, data)?),
        QueryMsg::QueryKeyHolders(data) => {
            to_binary(&query::key_holder::query_key_holders(deps, data)?)
        }
        QueryMsg::QueryUserHoldings(data) => {
            to_binary(&query::user_holding::query_user_holdings(deps, data)?)
        }
        QueryMsg::QuerySimulateBuyKey(data) => {
            to_binary(&query::key::query_simulate_buy_key(deps, data)?)
        }
        QueryMsg::QuerySimulateSellKey(data) => {
            to_binary(&query::key::query_simulate_sell_key(deps, data)?)
        }
        QueryMsg::QuerySimulateAsk(data) => to_binary(&query::thread::query_simulate_ask(deps, data)?),
    }
}
