use cosmwasm_std::{DepsMut, MessageInfo, Response};

use crate::state::CONFIG;
use crate::ContractError;

use distribution::msg::{
    AddToDistributeCallerAllowlistMsg, RemoveFromDistributeCallerAllowlistMsg, UpdateConfigMsg,
};

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

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn add_to_distribute_caller_allowlist(
    deps: DepsMut,
    info: MessageInfo,
    data: AddToDistributeCallerAllowlistMsg,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin_addr {
        return Err(ContractError::OnlyAdminCanAddToDistributionCallerAllowlist {});
    }

    config
        .distribute_caller_allowlist
        .push(deps.api.addr_validate(data.added_addr.as_str())?);

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "add_to_distribute_caller_allowlist"))
}

pub fn remove_from_distribute_caller_allowlist(
    deps: DepsMut,
    info: MessageInfo,
    data: RemoveFromDistributeCallerAllowlistMsg,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin_addr {
        return Err(ContractError::OnlyAdminCanAddToDistributionCallerAllowlist {});
    }

    let remove_addr = deps.api.addr_validate(data.remove_addr.as_str())?;

    let mut exist = false;
    let mut idx = 0;
    for (i, addr) in config.distribute_caller_allowlist.iter().enumerate() {
        if *addr == remove_addr {
            exist = true;
            idx = i;
            break;
        }
    }

    if !exist {
        return Err(ContractError::AddressNotInDistributionCallerAllowlist {});
    }

    config.distribute_caller_allowlist.remove(idx);

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "add_to_distribute_caller_allowlist"))
}
