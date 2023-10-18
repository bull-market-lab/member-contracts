use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint64,
};
use cw2::set_contract_version;

use member_pkg::{
    config::{Config, FeeConfig, ProtocolFeeConfig},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};
use shared_pkg::fee_share_config::FeeShareConfig;

use crate::state::{CONFIG, NEXT_USER_ID};
use crate::util::fee_share::assert_config_fee_share_sum_to_100;
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
        distribution_contract_addr: None,
        // Default to sender
        protocol_fee_collector_addr: deps.api.addr_validate(
            &msg.protocol_fee_collector_addr
                .unwrap_or(info.sender.to_string()),
        )?,
        default_fee_config: FeeConfig {
            fee_denom: msg.fee_denom.unwrap_or("uluna".to_string()),
            // By default, pay 5% of the total price of buying or selling amount of key to buy or sell
            trading_fee_percentage_of_membership: msg
                .default_trading_fee_percentage_of_membership
                .unwrap_or(Uint64::from(5_u64)),
        },
        protocol_fee_config: ProtocolFeeConfig {
            // Default to 10%
            // e.g. user pays 10 LUNA to buy 5 keys
            // Assume key issuer uses default_trading_fee_percentage_of_key which is 5%
            // And key issuer uses default_key_trading_fee_share_config which is 50% for key issuer and 50% for key holder
            // In total user pays 10.55 LUNA
            // 0.25 LUNA goes to key issuer, 0.25 LUNA gets splitted by all key holders proportionally
            // 0.05 (because 10% of 0.5 is 0.05) LUNA goes to protocol fee collector
            membership_trading_fee_percentage: msg
                .protocol_fee_membership_trading_fee_percentage
                .unwrap_or(Uint64::from(10_u64)),
        },
        default_fee_share_config: FeeShareConfig {
            // Default 80% goes to membership issuer
            share_to_issuer_percentage: msg
                .default_membership_trading_fee_membership_issuer_fee_percentage
                .unwrap_or(Uint64::from(80_u64)),
            // Default 20% goes to all members
            share_to_all_members_percentage: msg
                .default_membership_trading_fee_membership_holder_fee_percentage
                .unwrap_or(Uint64::from(20_u64)),
        },
    };

    NEXT_USER_ID.save(deps.storage, &Uint64::one())?;

    CONFIG.save(deps.storage, &config)?;
    assert_config_fee_share_sum_to_100(config.default_fee_share_config)?;

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
    // TODO: update this accordingly after user can set their own fee denom
    let fee_denom = config.default_fee_config.fee_denom.as_str();

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
        ExecuteMsg::UpdateUserConfig(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::update_user_config(deps, info, data)
        }
        ExecuteMsg::BuyMembership(data) => {
            let user_paid_amount = cw_utils::must_pay(&info, fee_denom)?;
            execute::member::buy_membership(
                deps,
                info,
                data,
                config.clone(),
                user_paid_amount,
                fee_denom.to_string(),
            )
        }
        ExecuteMsg::SellMembership(data) => {
            let user_paid_amount = cw_utils::must_pay(&info, fee_denom)?;
            execute::member::sell_membership(
                deps,
                info,
                data,
                config.clone(),
                user_paid_amount,
                fee_denom.to_string(),
            )
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    match msg {
        QueryMsg::QueryConfig(_) => to_binary(&query::config::query_config(deps)?),
        QueryMsg::QueryUserByAddr(data) => to_binary(&query::user::query_user_by_addr(deps, data)?),
        QueryMsg::QueryUserByID(data) => to_binary(&query::user::query_user_by_id(deps, data)?),
        QueryMsg::QueryUsersPaginatedByAddr(data) => {
            to_binary(&query::user::query_users_paginated_by_addr(deps, data)?)
        }
        QueryMsg::QueryUsersPaginatedByID(data) => {
            to_binary(&query::user::query_users_paginated_by_id(deps, data)?)
        }
        QueryMsg::QueryMembershipSupply(data) => {
            to_binary(&query::member::query_membership_supply(deps, data)?)
        }
        QueryMsg::QueryMemberCount(data) => {
            to_binary(&query::member::query_member_count(deps, data)?)
        }
        QueryMsg::QueryIsMember(data) => to_binary(&query::member::query_is_member(deps, data)?),
        QueryMsg::QueryMembers(data) => to_binary(&query::member::query_members(deps, data)?),
        QueryMsg::QueryMemberships(data) => {
            to_binary(&query::member::query_memberships(deps, data)?)
        }
        QueryMsg::QueryCostToBuyMembership(data) => to_binary(
            &query::cost::query_cost_to_buy_membership(deps, data, config)?,
        ),
        QueryMsg::QueryCostToSellMembership(data) => to_binary(
            &query::cost::query_cost_to_sell_membership(deps, data, config)?,
        ),
    }
}
