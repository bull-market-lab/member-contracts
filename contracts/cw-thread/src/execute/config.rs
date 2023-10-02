use crate::state::CONFIG;
use crate::ContractError;
use cosmwasm_std::{DepsMut, MessageInfo, Response};
use thread::msg::UpdateConfigMsg;

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

    config.registration_admin_addr = match data.registration_admin_addr {
        None => config.registration_admin_addr,
        Some(data) => deps.api.addr_validate(data.as_str())?,
    };

    config.protocol_fee_collector_addr = match data.protocol_fee_collector_addr {
        None => config.protocol_fee_collector_addr,
        Some(data) => deps.api.addr_validate(data.as_str())?,
    };

    config.fee_denom = data.fee_denom.unwrap_or(config.fee_denom);

    config.max_qa_thread_title_length = data
        .max_qa_thread_title_length
        .unwrap_or(config.max_qa_thread_title_length);

    config.max_qa_thread_msg_length = data
        .max_qa_thread_msg_length
        .unwrap_or(config.max_qa_thread_msg_length);

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_config")
        .add_attribute("config_admin_addr", config.admin_addr)
        .add_attribute(
            "config_registration_admin_addr",
            config.registration_admin_addr,
        )
        .add_attribute(
            "config_protocol_fee_collector_addr",
            config.protocol_fee_collector_addr,
        ))
}
