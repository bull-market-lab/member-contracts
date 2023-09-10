use crate::state::CONFIG;
use crate::ContractError;
use cosmwasm_std::{DepsMut, MessageInfo, Response};
use friend::msg::UpdateConfigMsg;

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateConfigMsg,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    config.admin = match data.admin {
        None => config.admin,
        Some(data) => deps.api.addr_validate(data.as_str())?,
    };

    config.key_register_admin = match data.key_register_admin {
        None => config.key_register_admin,
        Some(data) => deps.api.addr_validate(data.as_str())?,
    };

    config.fee_collector = match data.fee_collector {
        None => config.fee_collector,
        Some(data) => deps.api.addr_validate(data.as_str())?,
    };

    config.protocol_fee_percentage = data
        .protocol_fee_percentage
        .unwrap_or(config.protocol_fee_percentage);
    config.key_issuer_fee_percentage = data
        .key_issuer_fee_percentage
        .unwrap_or(config.key_issuer_fee_percentage);

    if config.protocol_fee_percentage.u64() > 100 {
        return Err(ContractError::ProtocolFeeTooHigh {});
    }

    if config.key_issuer_fee_percentage.u64() > 100 {
        return Err(ContractError::KeyIssuerFeeTooHigh {});
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_config")
        .add_attribute("config_admin", config.admin)
        .add_attribute("config_key_register_admin", config.key_register_admin)
        .add_attribute("config_fee_collector", config.fee_collector)
        .add_attribute(
            "config_protocol_fee_percentage",
            config.protocol_fee_percentage,
        )
        .add_attribute(
            "config_key_issuer_fee_percentage",
            config.key_issuer_fee_percentage,
        ))
}
