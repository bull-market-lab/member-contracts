use cosmwasm_std::{DepsMut, MessageInfo, Response, Uint128};

use friend::{key::Key, msg::RegisterSocialMediaAndKeyMsg, user::User};

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

pub fn register_social_media_and_key(
    deps: DepsMut,
    data: RegisterSocialMediaAndKeyMsg,
) -> Result<Response, ContractError> {
    let user_addr_ref = &data.user_addr;

    USERS.update(deps.storage, user_addr_ref, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            // User should not have a key yet
            if user.issued_key.is_some() {
                return Err(ContractError::UserAlreadyRegisteredKey {});
            }
            // User should not have a social media handle yet
            if user.social_media_handle.is_some() {
                return Err(ContractError::UserAlreadyVerifiedSocialMedia {});
            }
            let updated_user = User {
                addr: user.addr.clone(),
                social_media_handle: Some(data.social_media_handle.clone()),
                issued_key: Some(Key {
                    supply: Uint128::from(1 as u8),
                }),
            };
            Ok(updated_user)
        }
    })?;

    ALL_USERS_HOLDINGS.update(
        deps.storage,
        (user_addr_ref, user_addr_ref),
        |existing_holding| match existing_holding {
            // User should hold 1 key which issued by itself now
            None => Ok(Uint128::from(1 as u8)),
            // User should not have registered a key before
            Some(_) => Err(ContractError::UserAlreadyRegisteredKey {}),
        },
    )?;

    ALL_KEYS_HOLDERS.update(
        deps.storage,
        (user_addr_ref, user_addr_ref),
        |existing_holder| match existing_holder {
            // User's key should have 1 holder now which is itself
            None => Ok(Uint128::from(1 as u8)),
            // User should not have registered a key before
            Some(_) => Err(ContractError::UserAlreadyRegisteredKey {}),
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "register_social_media_and_key")
        .add_attribute("user_addr", data.user_addr)
        .add_attribute("social_media_handle", data.social_media_handle))
}
