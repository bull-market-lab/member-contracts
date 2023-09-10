use cosmwasm_std::{DepsMut, MessageInfo, Response, Uint128};

use friend::{
    key::Key, key_holder::KeyHolder, msg::RegisterSocialMediaAndKeyMsg, user::User,
    user_holding::UserHolding,
};

use crate::{
    state::{ALL_KEYS_HOLDERS, ALL_USERS_HOLDINGS, CONFIG, USERS},
    ContractError,
};

pub fn register(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    USERS.save(
        deps.storage,
        info.sender.clone(),
        &User {
            addr: info.sender.clone(),
            social_media_handle: None,
            issued_key: None,
        },
    )?;

    ALL_USERS_HOLDINGS.save(deps.storage, info.sender.clone(), &Vec::new())?;

    Ok(Response::new()
        .add_attribute("action", "register_user")
        .add_attribute("user_addr", info.sender))
}

pub fn register_social_media_and_key(
    deps: DepsMut,
    info: MessageInfo,
    data: RegisterSocialMediaAndKeyMsg,
) -> Result<Response, ContractError> {
    // Only key register admin can register key on behalf of user
    if info.sender != CONFIG.load(deps.storage)?.key_register_admin {
        return Err(ContractError::OnlyKeyRegisterAdminCanRegisterKeyOnBehalfOfUser {});
    }

    // User should exist in USER_HOLDINGS as it should be registered
    if !ALL_USERS_HOLDINGS.has(deps.storage, data.user_addr.clone()) {
        return Err(ContractError::UserNotExist {});
    }

    // User should not be in KEY_HOLDINGS as it should not have a key yet
    if ALL_KEYS_HOLDERS.has(deps.storage, data.user_addr.clone()) {
        return Err(ContractError::UserAlreadyRegisteredKey {});
    }

    USERS.update(deps.storage, data.user_addr.clone(), |user| match user {
        // User should exist in USERS as it should be registered
        None => {
            return Err(ContractError::UserNotExist {});
        }
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

    // User should hold 1 key itself now
    ALL_USERS_HOLDINGS
        .load(deps.storage, data.user_addr.clone())?
        .push(UserHolding {
            issuer_addr: data.user_addr.clone(),
            amount: Uint128::from(1 as u8),
        });

    // User's key should have 1 holder now which is itself
    ALL_KEYS_HOLDERS.save(
        deps.storage,
        data.user_addr.clone(),
        &vec![KeyHolder {
            holder_addr: data.user_addr.clone(),
            amount: Uint128::from(1 as u8),
        }],
    )?;

    Ok(Response::new()
        .add_attribute("action", "register_social_media_and_key")
        .add_attribute("user_addr", data.user_addr)
        .add_attribute("social_media_handle", data.social_media_handle))
}
