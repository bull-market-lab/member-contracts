use cosmwasm_std::{DepsMut, MessageInfo, Response};
use member_pkg::config::{FeeConfig, ProtocolFeeConfig};
use shared_pkg::fee_share_config::FeeShareConfig;

use crate::ContractError;
use crate::{state::CONFIG, util::fee_share::assert_config_fee_share_sum_to_100};

use member_pkg::msg::UpdateConfigMsg;

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

pub fn enable_open_registration(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin_addr {
        return Err(ContractError::OnlyAdminCanEnableOpenRegistration {});
    }

    config.enable_open_registration = true;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "enable_open_registration"))
}

pub fn disable_open_registration(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin_addr {
        return Err(ContractError::OnlyAdminCanDisableOpenRegistration {});
    }

    config.enable_open_registration = false;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "disable_open_registration"))
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

    config.registration_admin_addr = match data.registration_admin_addr {
        None => config.registration_admin_addr,
        Some(data) => deps.api.addr_validate(data.as_str())?,
    };

    config.distribution_contract_addr = match data.distribution_contract_addr {
        None => config.distribution_contract_addr,
        Some(data) => Some(deps.api.addr_validate(data.as_str())?),
    };

    config.protocol_fee_collector_addr = match data.protocol_fee_collector_addr {
        None => config.protocol_fee_collector_addr,
        Some(data) => deps.api.addr_validate(data.as_str())?,
    };

    config.protocol_fee_config = ProtocolFeeConfig {
        membership_trading_fee_percentage: data
            .protocol_fee_membership_trading_fee_percentage
            .unwrap_or(config.protocol_fee_config.membership_trading_fee_percentage),
    };

    config.default_fee_config = FeeConfig {
        fee_denom: config.default_fee_config.fee_denom,
        trading_fee_percentage_of_membership: data
            .default_trading_fee_percentage_of_membership
            .unwrap_or(
                config
                    .default_fee_config
                    .trading_fee_percentage_of_membership,
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
    assert_config_fee_share_sum_to_100(config.default_fee_share_config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}
