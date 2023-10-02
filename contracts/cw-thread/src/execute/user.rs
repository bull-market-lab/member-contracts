use cosmwasm_std::{DepsMut, MessageInfo, Response, Uint128};

use thread::{
    config::Config,
    key::Key,
    msg::{LinkSocialMediaMsg, RegisterKeyMsg, UpdateThreadFeeConfigMsg, UpdateTradingFeeConfigMsg},
    user::User,
};

use crate::{
    state::{ALL_KEYS_HOLDERS, ALL_USERS_HOLDINGS, USERS},
    ContractError,
};

pub fn register(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    USERS.save(
        deps.storage,
        &info.sender,
        &User {
            addr: info.sender.clone(),
            social_media_handle: None,
            issued_key: None,
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "register_user")
        .add_attribute("user_addr", info.sender))
}

pub fn link_social_media(
    deps: DepsMut,
    info: MessageInfo,
    data: LinkSocialMediaMsg,
    config: Config,
) -> Result<Response, ContractError> {
    if info.sender != config.registration_admin_addr {
        return Err(ContractError::OnlyRegistrationAdminCanLinkSocialMediaOnBehalfOfUser {});
    }

    let user_addr_ref = &data.user_addr;

    USERS.update(deps.storage, user_addr_ref, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            // User should not have linked a social media handle yet
            if user.social_media_handle.is_some() {
                return Err(ContractError::UserAlreadyLinkedSocialMedia {});
            }
            let updated_user = User {
                addr: user.addr,
                social_media_handle: Some(data.social_media_handle.clone()),
                issued_key: user.issued_key,
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "link_social_media")
        .add_attribute("user_addr", data.user_addr)
        .add_attribute("social_media_handle", data.social_media_handle))
}

pub fn register_key(
    deps: DepsMut,
    info: MessageInfo,
    data: RegisterKeyMsg,
    config: Config,
) -> Result<Response, ContractError> {
    if info.sender != config.registration_admin_addr {
        return Err(ContractError::OnlyRegistrationAdminCanRegisterKeyOnBehalfOfUser {});
    }

    let user_addr_ref = &data.user_addr;

    USERS.update(deps.storage, user_addr_ref, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            // User should not have a key yet
            if user.issued_key.is_some() {
                return Err(ContractError::UserAlreadyRegisteredKey {});
            }
            // User must already have a social media handle
            if user.social_media_handle.is_none() {
                return Err(ContractError::UserCannotRegisterKeyBeforeLinkingSocialMedia {});
            }
            let updated_user = User {
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                issued_key: Some(Key {
                    supply: Uint128::one(),
                    key_trading_fee_config: data.key_trading_fee_config,
                    thread_fee_config: data.thread_fee_config,
                }),
            };
            Ok(updated_user)
        }
    })?;

    ALL_USERS_HOLDINGS.update(
        deps.storage,
        (user_addr_ref, user_addr_ref),
        |existing_holding| match existing_holding {
            // User should not hold its own key because it has never registered a key before
            Some(_) => Err(ContractError::UserAlreadyRegisteredKey {}),
            // User should hold 1 new key which is issued by itself now
            None => Ok(Uint128::one()),
        },
    )?;

    ALL_KEYS_HOLDERS.update(
        deps.storage,
        (user_addr_ref, user_addr_ref),
        |existing_holder| match existing_holder {
            // User's key should not have any holder before because it has never registered a key before
            Some(_) => Err(ContractError::UserAlreadyRegisteredKey {}),
            // User's key should have 1 holder now which is itself
            None => Ok(Uint128::one()),
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "register_key")
        .add_attribute("user_addr", data.user_addr))
}

pub fn update_trading_fee_config(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateTradingFeeConfigMsg,
) -> Result<Response, ContractError> {
    if info.sender != USERS.load(deps.storage, &data.key_issuer_addr)?.addr {
        return Err(ContractError::OnlyKeyIssuerCanUpdateItsTradingFeeConfig {});
    }

    USERS.update(deps.storage, &data.key_issuer_addr, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            // User should have a key
            if user.issued_key.is_none() {
                return Err(ContractError::UserHasNotRegisteredKey {});
            }
            let updated_user = User {
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                issued_key: Some(Key {
                    supply: user.issued_key.clone().unwrap().supply,
                    key_trading_fee_config: data.key_trading_fee_config,
                    thread_fee_config: user.issued_key.unwrap().thread_fee_config,
                }),
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_trading_fee_config")
        .add_attribute("key_issuer_addr", data.key_issuer_addr))
}

pub fn update_thread_fee_config(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateThreadFeeConfigMsg,
) -> Result<Response, ContractError> {
    if info.sender != USERS.load(deps.storage, &data.key_issuer_addr)?.addr {
        return Err(ContractError::OnlyKeyIssuerCanUpdateItsThreadFeeConfig {});
    }

    USERS.update(deps.storage, &data.key_issuer_addr, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            // User should have a key
            if user.issued_key.is_none() {
                return Err(ContractError::UserHasNotRegisteredKey {});
            }
            let updated_user = User {
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                issued_key: Some(Key {
                    supply: user.issued_key.clone().unwrap().supply,
                    key_trading_fee_config: user.issued_key.unwrap().key_trading_fee_config,
                    thread_fee_config: data.thread_fee_config,
                }),
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_thread_fee_config")
        .add_attribute("key_issuer_addr", data.key_issuer_addr))
}
