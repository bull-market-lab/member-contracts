use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint64,
};

use membership::config::Config;
use membership::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

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
        enabled: false,
        enable_open_registration: false,
        // Default to sender
        admin_addr: deps
            .api
            .addr_validate(&msg.admin_addr.unwrap_or(info.sender.to_string()))?,
        // Default to sender
        registration_admin_addr: deps.api.addr_validate(
            &msg.registration_admin_addr
                .unwrap_or(info.sender.to_string()),
        )?,
        // Default to sender
        protocol_fee_collector_addr: deps.api.addr_validate(
            &msg.protocol_fee_collector_addr
                .unwrap_or(info.sender.to_string()),
        )?,
        fee_denom: msg.fee_denom.unwrap_or("uluna".to_string()),

        // By default, pay 5% of the total price of buying or selling amount of key to buy or sell
        default_trading_fee_percentage_of_membership: msg
            .default_trading_fee_percentage_of_membership
            .unwrap_or(Uint64::from(5_u64)),

        // Default to 10%
        // e.g. user pays 10 LUNA to buy 5 keys
        // Assume key issuer uses default_trading_fee_percentage_of_key which is 5%
        // And key issuer uses default_key_trading_fee_share_config which is 50% for key issuer and 50% for key holder
        // In total user pays 10.55 LUNA
        // 0.25 LUNA goes to key issuer, 0.25 LUNA gets splitted by all key holders proportionally
        // 0.05 (because 10% of 0.5 is 0.05) LUNA goes to protocol fee collector
        protocol_fee_membership_trading_fee_percentage: msg
            .protocol_fee_membership_trading_fee_percentage
            .unwrap_or(Uint64::from(10_u64)),

        // Default to 50%
        default_share_to_issuer_percentage: msg
            .default_membership_trading_fee_membership_issuer_fee_percentage
            .unwrap_or(Uint64::from(50_u64)),
        // Default to 50%
        default_share_to_all_members_percentage: msg
            .default_membership_trading_fee_membership_holder_fee_percentage
            .unwrap_or(Uint64::from(50_u64)),
    };

    if config.default_share_to_issuer_percentage + config.default_share_to_all_members_percentage
        != Uint64::from(100_u64)
    {
        return Err(ContractError::MembershipTradingFeeSharePercentageMustBe100 {});
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
    let config = CONFIG.load(deps.storage)?;
    match msg {
        ExecuteMsg::Enable(_) => {
            cw_utils::nonpayable(&info)?;
            execute::config::enable(deps, info)
        }
        ExecuteMsg::Disable(_) => {
            cw_utils::nonpayable(&info)?;
            execute::config::disable(deps, info)
        }
        ExecuteMsg::EnableOpenRegistration(_) => {
            cw_utils::nonpayable(&info)?;
            execute::config::enable_open_registration(deps, info)
        }
        ExecuteMsg::DisableOpenRegistration(_) => {
            cw_utils::nonpayable(&info)?;
            execute::config::disable_open_registration(deps, info)
        }
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
        ExecuteMsg::EnableMembership(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::enable_membership(deps, info, data, config)
        }
        ExecuteMsg::UpdateTradingFeePercentageOfMembership(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::update_trading_fee_percentage_of_membership(deps, info, data)
        }
        ExecuteMsg::UpdateMembershipTradingFeeShareConfig(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::update_membership_trading_fee_share_config(deps, info, data)
        }
        ExecuteMsg::BuyMembership(data) => {
            let user_paid_amount = cw_utils::must_pay(&info, config.fee_denom.as_str())?;
            execute::membership::buy_membership(deps, env, info, data, config, user_paid_amount)
        }
        ExecuteMsg::SellMembership(data) => {
            let user_paid_amount = cw_utils::must_pay(&info, config.fee_denom.as_str())?;
            execute::membership::sell_membership(deps, env, info, data, config, user_paid_amount)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryConfig(_) => to_binary(&query::config::query_config(deps)?),
        QueryMsg::QueryUser(data) => to_binary(&query::user::query_user(deps, data)?),
        QueryMsg::QueryMembershipSupply(data) => {
            to_binary(&query::membership::query_membership_supply(deps, data)?)
        }
        QueryMsg::QueryMemberCount(data) => {
            to_binary(&query::membership::query_member_count(deps, data)?)
        }
        QueryMsg::QueryMembers(data) => to_binary(&query::membership::query_members(deps, data)?),
        QueryMsg::QueryMemberships(data) => {
            to_binary(&query::membership::query_memberships(deps, data)?)
        }
        QueryMsg::QueryCostToBuyMembership(data) => {
            to_binary(&query::cost::query_cost_to_buy_membership(deps, data)?)
        }
        QueryMsg::QueryCostToSellMembership(data) => {
            to_binary(&query::cost::query_cost_to_sell_membership(deps, data)?)
        }
    }
}
