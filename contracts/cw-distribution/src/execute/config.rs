use cosmwasm_std::{DepsMut, MessageInfo, Response};

use crate::state::CONFIG;
use crate::ContractError;

use distribution::msg::UpdateConfigMsg;

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

    config.membership_contract_addr = match data.membership_contract_addr {
        None => config.membership_contract_addr,
        Some(data) => deps.api.addr_validate(data.as_str())?,
    };

    // config.minimum_eligible_weight = match data.minimum_eligible_weight {
    //     None => config.minimum_eligible_weight,
    //     Some(data) => data,
    // };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}
