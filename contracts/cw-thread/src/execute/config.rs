use cosmwasm_std::{DepsMut, MessageInfo, Response};
use shared::fee_share_config::FeeShareConfig;

use crate::{state::CONFIG, util::fee_share::assert_config_fee_share_sum_to_100, ContractError};

use thread::config::{FeeConfig, ProtocolFeeConfig, ThreadConfig};
use thread::msg::UpdateConfigMsg;

pub fn enable(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin_addr {
        return Err(ContractError::OnlyAdminCanEnable {});
    }

    config.enabled = true;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "enable"))
}

pub fn disable(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin_addr {
        return Err(ContractError::OnlyAdminCanDisable {});
    }

    config.enabled = false;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "disable"))
}

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateConfigMsg,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin_addr {
        return Err(ContractError::OnlyAdminCanUpdateConfig {});
    }

    config.admin_addr = match data.admin_addr {
        None => config.admin_addr,
        Some(data) => deps.api.addr_validate(data.as_str())?,
    };

    config.protocol_fee_collector_addr = match data.protocol_fee_collector_addr {
        None => config.protocol_fee_collector_addr,
        Some(data) => deps.api.addr_validate(data.as_str())?,
    };

    config.membership_contract_addr = match data.membership_contract_addr {
        None => config.membership_contract_addr,
        Some(data) => deps.api.addr_validate(data.as_str())?,
    };

    config.thread_config = ThreadConfig {
        max_thread_title_length: data
            .max_thread_title_length
            .unwrap_or(config.thread_config.max_thread_title_length),
        max_thread_description_length: data
            .max_thread_description_length
            .unwrap_or(config.thread_config.max_thread_description_length),
        max_thread_label_length: data
            .max_thread_label_length
            .unwrap_or(config.thread_config.max_thread_label_length),
        max_number_of_thread_labels: data
            .max_number_of_thread_labels
            .unwrap_or(config.thread_config.max_number_of_thread_labels),
        max_thread_msg_length: data
            .max_thread_msg_length
            .unwrap_or(config.thread_config.max_thread_msg_length),
    };

    config.protocol_fee_config = ProtocolFeeConfig {
        start_new_thread_fixed_cost: data
            .protocol_fee_start_new_thread_fixed_cost
            .unwrap_or(config.protocol_fee_config.start_new_thread_fixed_cost),
        ask_in_thread_fee_percentage: data
            .protocol_fee_ask_in_thread_fee_percentage
            .unwrap_or(config.protocol_fee_config.ask_in_thread_fee_percentage),
        reply_in_thread_fee_percentage: data
            .protocol_fee_reply_in_thread_fee_percentage
            .unwrap_or(config.protocol_fee_config.reply_in_thread_fee_percentage),
    };

    config.default_fee_config = FeeConfig {
        ask_fee_percentage_of_membership: data
            .default_ask_fee_percentage_of_membership
            .unwrap_or(config.default_fee_config.ask_fee_percentage_of_membership),
        ask_fee_to_thread_creator_percentage_of_membership: data
            .default_ask_fee_to_thread_creator_percentage_of_membership
            .unwrap_or(
                config
                    .default_fee_config
                    .ask_fee_to_thread_creator_percentage_of_membership,
            ),
        reply_fee_percentage_of_membership: data
            .default_reply_fee_percentage_of_membership
            .unwrap_or(config.default_fee_config.reply_fee_percentage_of_membership),
        reply_fee_to_thread_creator_percentage_of_membership: data
            .default_reply_fee_to_thread_creator_percentage_of_membership
            .unwrap_or(
                config
                    .default_fee_config
                    .reply_fee_to_thread_creator_percentage_of_membership,
            ),
    };

    config.default_fee_share_config = FeeShareConfig {
        share_to_issuer_percentage: data
            .default_share_to_issuer_percentage
            .unwrap_or(config.default_fee_share_config.share_to_issuer_percentage),
        share_to_all_members_percentage: data.default_share_to_all_members_percentage.unwrap_or(
            config
                .default_fee_share_config
                .share_to_all_members_percentage,
        ),
    };

    CONFIG.save(deps.storage, &config)?;
    assert_config_fee_share_sum_to_100(deps.as_ref())?;

    Ok(Response::new().add_attribute("action", "update_config"))
}
