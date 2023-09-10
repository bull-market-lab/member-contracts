use crate::state::CONFIG;
use crate::ContractError;
use cosmwasm_std::{DepsMut, Response};
use friend::msg::UpdateConfigMsg;

pub fn update_config(deps: DepsMut, data: UpdateConfigMsg) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    config.admin_addr = match data.admin_addr {
        None => config.admin_addr,
        Some(data) => deps.api.addr_validate(data.as_str())?,
    };

    config.key_register_admin_addr = match data.key_register_admin_addr {
        None => config.key_register_admin_addr,
        Some(data) => deps.api.addr_validate(data.as_str())?,
    };

    config.protocol_fee_collector_addr = match data.protocol_fee_collector_addr {
        None => config.protocol_fee_collector_addr,
        Some(data) => deps.api.addr_validate(data.as_str())?,
    };

    config.fee_denom = data.fee_denom.unwrap_or(config.fee_denom);

    config.protocol_fee_percentage = data
        .protocol_fee_percentage
        .unwrap_or(config.protocol_fee_percentage);
    config.key_issuer_fee_percentage = data
        .key_issuer_fee_percentage
        .unwrap_or(config.key_issuer_fee_percentage);

    if config.protocol_fee_percentage.u128() > 100 {
        return Err(ContractError::ProtocolFeeTooHigh {});
    }

    if config.key_issuer_fee_percentage.u128() > 100 {
        return Err(ContractError::KeyIssuerFeeTooHigh {});
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_config")
        .add_attribute("config_admin_addr", config.admin_addr)
        .add_attribute(
            "config_key_register_admin_addr",
            config.key_register_admin_addr,
        )
        .add_attribute(
            "config_protocol_fee_collector_addr",
            config.protocol_fee_collector_addr,
        )
        .add_attribute(
            "config_protocol_fee_percentage",
            config.protocol_fee_percentage,
        )
        .add_attribute(
            "config_key_issuer_fee_percentage",
            config.key_issuer_fee_percentage,
        ))
}
