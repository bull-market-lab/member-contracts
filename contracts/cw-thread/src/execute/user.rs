use cosmwasm_std::{DepsMut, MessageInfo, Response, Uint128};

use thread::{
    config::Config,
    msg::{
        LinkSocialMediaMsg, RegisterMembershipMsg, UpdateAskFeePercentageOfMembershipMsg,
        UpdateAskFeeToThreadCreatorPercentageOfMembershipMsg,
        UpdateMembershipTradingFeeShareConfigMsg, UpdateReplyFeePercentageOfMembershipMsg,
        UpdateThreadFeeShareConfigMsg, UpdateTradingFeePercentageOfMembershipMsg,
    },
    user::User,
};

use crate::{
    state::{ALL_MEMBERSHIPS_MEMBERS, ALL_USERS_MEMBERSHIPS, MEMBERSHIP_SUPPLY, USERS},
    ContractError,
};

pub fn register(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    USERS.save(
        deps.storage,
        &info.sender,
        &User {
            addr: info.sender.clone(),
            social_media_handle: None,
            issued_key: false,
            trading_fee_percentage_of_key: None,
            ask_fee_percentage_of_key: None,
            ask_fee_to_thread_creator_percentage_of_key: None,
            reply_fee_percentage_of_key: None,
            key_trading_fee_share_config: None,
            thread_fee_share_config: None,
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

    let user_addr_ref = &deps.api.addr_validate(data.user_addr.as_str()).unwrap();

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
                trading_fee_percentage_of_key: user.trading_fee_percentage_of_key,
                ask_fee_percentage_of_key: user.ask_fee_percentage_of_key,
                ask_fee_to_thread_creator_percentage_of_key: user
                    .ask_fee_to_thread_creator_percentage_of_key,
                reply_fee_percentage_of_key: user.reply_fee_percentage_of_key,
                key_trading_fee_share_config: user.key_trading_fee_share_config,
                thread_fee_share_config: user.thread_fee_share_config,
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "link_social_media")
        .add_attribute("user_addr", data.user_addr)
        .add_attribute("social_media_handle", data.social_media_handle))
}

pub fn enable_membership(
    deps: DepsMut,
    info: MessageInfo,
    data: RegisterMembershipMsg,
    config: Config,
) -> Result<Response, ContractError> {
    if info.sender != config.registration_admin_addr {
        return Err(ContractError::OnlyRegistrationAdminCanRegisterMembershipOnBehalfOfUser {});
    }

    let user_addr_ref = &deps.api.addr_validate(data.user_addr.as_str()).unwrap();

    let user = USERS.load(deps.storage, user_addr_ref)?;

    // User should not have a key yet
    if user.issued_key {
        return Err(ContractError::UserAlreadyRegisteredMembership {});
    }

    // User must already have a social media handle
    if user.social_media_handle.is_none() {
        return Err(ContractError::UserCannotRegisterMembershipBeforeLinkingSocialMedia {});
    }

    USERS.update(deps.storage, user_addr_ref, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            let updated_user = User {
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                issued_key: true,
                trading_fee_percentage_of_key: user.trading_fee_percentage_of_key,
                ask_fee_percentage_of_key: user.ask_fee_percentage_of_key,
                ask_fee_to_thread_creator_percentage_of_key: user
                    .ask_fee_to_thread_creator_percentage_of_key,
                reply_fee_percentage_of_key: user.reply_fee_percentage_of_key,
                key_trading_fee_share_config: user.key_trading_fee_share_config,
                thread_fee_share_config: user.thread_fee_share_config,
            };
            Ok(updated_user)
        }
    })?;

    MEMBERSHIP_SUPPLY.update(deps.storage, user_addr_ref, |supply| match supply {
        None => Ok(Uint128::one()),
        // User should not have any key before because it has never registered a key before
        Some(_) => Err(ContractError::UserAlreadyRegisteredMembership {}),
    })?;

    ALL_USERS_MEMBERSHIPS.update(
        deps.storage,
        (user_addr_ref, user_addr_ref),
        |existing_holding| match existing_holding {
            // User should not hold its own key because it has never registered a key before
            Some(_) => Err(ContractError::UserAlreadyRegisteredMembership {}),
            // User should hold 1 new key which is issued by itself now
            None => Ok(Uint128::one()),
        },
    )?;

    ALL_MEMBERSHIPS_MEMBERS.update(
        deps.storage,
        (user_addr_ref, user_addr_ref),
        |existing_holder| match existing_holder {
            // User's key should not have any holder before because it has never registered a key before
            Some(_) => Err(ContractError::UserAlreadyRegisteredMembership {}),
            // User's key should have 1 holder now which is itself
            None => Ok(Uint128::one()),
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "enable_membership")
        .add_attribute("user_addr", data.user_addr))
}

pub fn update_trading_fee_percentage_of_membership(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateTradingFeePercentageOfMembershipMsg,
) -> Result<Response, ContractError> {
    let key_issuer_addr_ref = &deps
        .api
        .addr_validate(data.key_issuer_addr.as_str())
        .unwrap();
    if info.sender != USERS.load(deps.storage, key_issuer_addr_ref)?.addr {
        return Err(
            ContractError::OnlyMembershipIssuerCanUpdateItsTradingFeePercentageOfMembership {},
        );
    }

    USERS.update(deps.storage, key_issuer_addr_ref, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            let updated_user = User {
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                issued_key: user.issued_key,
                trading_fee_percentage_of_key: Some(data.trading_fee_percentage_of_key),
                ask_fee_percentage_of_key: user.ask_fee_percentage_of_key,
                ask_fee_to_thread_creator_percentage_of_key: user
                    .ask_fee_to_thread_creator_percentage_of_key,
                reply_fee_percentage_of_key: user.reply_fee_percentage_of_key,
                key_trading_fee_share_config: user.key_trading_fee_share_config,
                thread_fee_share_config: user.thread_fee_share_config,
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_trading_fee_percentage_of_membership")
        .add_attribute("key_issuer_addr", data.key_issuer_addr))
}

pub fn update_ask_fee_percentage_of_key(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateAskFeePercentageOfMembershipMsg,
) -> Result<Response, ContractError> {
    let key_issuer_addr_ref = &deps
        .api
        .addr_validate(data.key_issuer_addr.as_str())
        .unwrap();
    if info.sender != USERS.load(deps.storage, key_issuer_addr_ref)?.addr {
        return Err(ContractError::OnlyMembershipIssuerCanUpdateItsAskFeePercentageOfMembership {});
    }

    USERS.update(deps.storage, key_issuer_addr_ref, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            let updated_user = User {
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                issued_key: user.issued_key,
                trading_fee_percentage_of_key: user.trading_fee_percentage_of_key,
                ask_fee_percentage_of_key: Some(data.ask_fee_percentage_of_key),
                ask_fee_to_thread_creator_percentage_of_key: user
                    .ask_fee_to_thread_creator_percentage_of_key,
                reply_fee_percentage_of_key: user.reply_fee_percentage_of_key,
                key_trading_fee_share_config: user.key_trading_fee_share_config,
                thread_fee_share_config: user.thread_fee_share_config,
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_ask_fee_percentage_of_key")
        .add_attribute("key_issuer_addr", data.key_issuer_addr))
}

pub fn update_ask_fee_to_thread_creator_percentage_of_key(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateAskFeeToThreadCreatorPercentageOfMembershipMsg,
) -> Result<Response, ContractError> {
    let key_issuer_addr_ref = &deps
        .api
        .addr_validate(data.key_issuer_addr.as_str())
        .unwrap();
    if info.sender != USERS.load(deps.storage, key_issuer_addr_ref)?.addr {
        return Err(
            ContractError::OnlyMembershipIssuerCanUpdateItsAskFeeToCreatorPercentageOfMembership {},
        );
    }

    USERS.update(deps.storage, key_issuer_addr_ref, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            let updated_user = User {
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                issued_key: user.issued_key,
                trading_fee_percentage_of_key: user.trading_fee_percentage_of_key,
                ask_fee_percentage_of_key: user.ask_fee_percentage_of_key,
                ask_fee_to_thread_creator_percentage_of_key: Some(
                    data.ask_fee_to_thread_creator_percentage_of_key,
                ),
                reply_fee_percentage_of_key: user.reply_fee_percentage_of_key,
                key_trading_fee_share_config: user.key_trading_fee_share_config,
                thread_fee_share_config: user.thread_fee_share_config,
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute(
            "action",
            "update_ask_fee_to_thread_creator_percentage_of_key",
        )
        .add_attribute("key_issuer_addr", data.key_issuer_addr))
}

pub fn update_reply_fee_percentage_of_key(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateReplyFeePercentageOfMembershipMsg,
) -> Result<Response, ContractError> {
    let key_issuer_addr_ref = &deps
        .api
        .addr_validate(data.key_issuer_addr.as_str())
        .unwrap();
    if info.sender != USERS.load(deps.storage, key_issuer_addr_ref)?.addr {
        return Err(
            ContractError::OnlyMembershipIssuerCanUpdateItsReplyFeePercentageOfMembership {},
        );
    }

    USERS.update(deps.storage, key_issuer_addr_ref, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            let updated_user = User {
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                issued_key: user.issued_key,
                trading_fee_percentage_of_key: user.trading_fee_percentage_of_key,
                ask_fee_percentage_of_key: user.ask_fee_percentage_of_key,
                ask_fee_to_thread_creator_percentage_of_key: user
                    .ask_fee_to_thread_creator_percentage_of_key,
                reply_fee_percentage_of_key: Some(data.reply_fee_percentage_of_key),
                key_trading_fee_share_config: user.key_trading_fee_share_config,
                thread_fee_share_config: user.thread_fee_share_config,
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_reply_fee_percentage_of_key")
        .add_attribute("key_issuer_addr", data.key_issuer_addr))
}

pub fn update_membership_trading_fee_share_config(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateMembershipTradingFeeShareConfigMsg,
) -> Result<Response, ContractError> {
    let key_issuer_addr_ref = &deps
        .api
        .addr_validate(data.key_issuer_addr.as_str())
        .unwrap();

    if info.sender != USERS.load(deps.storage, key_issuer_addr_ref)?.addr {
        return Err(ContractError::OnlyMembershipIssuerCanUpdateItsTradingFeeConfig {});
    }

    USERS.update(deps.storage, key_issuer_addr_ref, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            let updated_user = User {
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                issued_key: user.issued_key,
                trading_fee_percentage_of_key: user.trading_fee_percentage_of_key,
                ask_fee_percentage_of_key: user.ask_fee_percentage_of_key,
                ask_fee_to_thread_creator_percentage_of_key: user
                    .ask_fee_to_thread_creator_percentage_of_key,
                reply_fee_percentage_of_key: user.reply_fee_percentage_of_key,
                key_trading_fee_share_config: Some(data.key_trading_fee_share_config),
                thread_fee_share_config: user.thread_fee_share_config,
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_trading_fee_config")
        .add_attribute("key_issuer_addr", data.key_issuer_addr))
}

pub fn update_thread_fee_share_config(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateThreadFeeShareConfigMsg,
) -> Result<Response, ContractError> {
    let key_issuer_addr_ref = &deps
        .api
        .addr_validate(data.key_issuer_addr.as_str())
        .unwrap();

    if info.sender != USERS.load(deps.storage, key_issuer_addr_ref)?.addr {
        return Err(ContractError::OnlyMembershipIssuerCanUpdateItsThreadFeeConfig {});
    }

    USERS.update(deps.storage, key_issuer_addr_ref, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            let updated_user = User {
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                issued_key: user.issued_key,
                trading_fee_percentage_of_key: user.trading_fee_percentage_of_key,
                ask_fee_percentage_of_key: user.ask_fee_percentage_of_key,
                ask_fee_to_thread_creator_percentage_of_key: user
                    .ask_fee_to_thread_creator_percentage_of_key,
                reply_fee_percentage_of_key: user.reply_fee_percentage_of_key,
                key_trading_fee_share_config: user.key_trading_fee_share_config,
                thread_fee_share_config: Some(data.thread_fee_share_config),
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_thread_fee_config")
        .add_attribute("key_issuer_addr", data.key_issuer_addr))
}
