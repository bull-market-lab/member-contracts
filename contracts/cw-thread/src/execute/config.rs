use crate::state::CONFIG;
use crate::ContractError;
use cosmwasm_std::{DepsMut, MessageInfo, Response, Uint64};
use thread::msg::{UpdateConfigMsg, UpdateMembershipContractAddrMsg};

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

pub fn update_membership_contract_addr(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateMembershipContractAddrMsg,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin_addr {
        return Err(ContractError::OnlyAdminCanDisable {});
    }

    config.membership_contract_addr = deps.api.addr_validate(&data.membership_contract_addr)?;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_membership_contract_addr")
        .add_attribute(
            "update_membership_contract_addr",
            data.membership_contract_addr,
        ))
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

    config.max_thread_title_length = data
        .max_thread_title_length
        .unwrap_or(config.max_thread_title_length);

    config.max_thread_description_length = data
        .max_thread_description_length
        .unwrap_or(config.max_thread_description_length);

    config.max_thread_msg_length = data
        .max_thread_msg_length
        .unwrap_or(config.max_thread_msg_length);

    config.max_thread_label_length = data
        .max_thread_label_length
        .unwrap_or(config.max_thread_label_length);

    config.max_number_of_thread_labels = data
        .max_number_of_thread_labels
        .unwrap_or(config.max_number_of_thread_labels);

    config.protocol_fee_start_new_thread_fixed_cost = data
        .protocol_fee_start_new_thread_fixed_cost
        .unwrap_or(config.protocol_fee_start_new_thread_fixed_cost);

    config.protocol_fee_ask_in_thread_fee_percentage = data
        .protocol_fee_ask_in_thread_fee_percentage
        .unwrap_or(config.protocol_fee_ask_in_thread_fee_percentage);

    config.protocol_fee_reply_in_thread_fee_percentage = data
        .protocol_fee_reply_in_thread_fee_percentage
        .unwrap_or(config.protocol_fee_reply_in_thread_fee_percentage);

    config.default_ask_fee_percentage_of_membership = data
        .default_ask_fee_percentage_of_membership
        .unwrap_or(config.default_ask_fee_percentage_of_membership);

    config.default_ask_fee_to_thread_creator_percentage_of_membership = data
        .default_ask_fee_to_thread_creator_percentage_of_membership
        .unwrap_or(config.default_ask_fee_to_thread_creator_percentage_of_membership);

    config.default_reply_fee_percentage_of_membership = data
        .default_reply_fee_percentage_of_membership
        .unwrap_or(config.default_reply_fee_percentage_of_membership);

    config.default_share_to_issuer_percentage = data
        .default_share_to_issuer_percentage
        .unwrap_or(config.default_share_to_issuer_percentage);

    config.default_share_to_all_members_percentage = data
        .default_share_to_all_members_percentage
        .unwrap_or(config.default_share_to_all_members_percentage);

    if config.default_share_to_issuer_percentage + config.default_share_to_all_members_percentage
        != Uint64::from(100_u64)
    {
        return Err(ContractError::ThreadFeeSharePercentageMustSumTo100 {});
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}
