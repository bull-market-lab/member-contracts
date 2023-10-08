use cosmwasm_std::{DepsMut, MessageInfo, Response};

use thread::{
    config::Config,
    msg::{
        UpdateAskFeePercentageOfMembershipMsg,
        UpdateAskFeeToThreadCreatorPercentageOfMembershipMsg,
        UpdateReplyFeePercentageOfMembershipMsg, UpdateThreadFeeShareConfigMsg,
    },
    user::User,
};

use crate::{state::USERS, ContractError};

pub fn update_trading_fee_percentage_of_membership(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateTradingFeePercentageOfMembershipMsg,
) -> Result<Response, ContractError> {
    let membership_issuer_addr_ref = &deps
        .api
        .addr_validate(data.membership_issuer_addr.as_str())
        .unwrap();
    if info.sender != USERS.load(deps.storage, membership_issuer_addr_ref)?.addr {
        return Err(
            ContractError::OnlyMembershipIssuerCanUpdateItsTradingFeePercentageOfMembership {},
        );
    }

    USERS.update(deps.storage, membership_issuer_addr_ref, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            let updated_user = User {
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                issued_membership: user.issued_membership,
                trading_fee_percentage_of_membership: Some(data.trading_fee_percentage_of_membership),
                ask_fee_percentage_of_membership: user.ask_fee_percentage_of_membership,
                ask_fee_to_thread_creator_percentage_of_membership: user
                    .ask_fee_to_thread_creator_percentage_of_membership,
                reply_fee_percentage_of_membership: user.reply_fee_percentage_of_membership,
                membership_trading_fee_share_config: user.membership_trading_fee_share_config,
                thread_fee_share_config: user.thread_fee_share_config,
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_trading_fee_percentage_of_membership")
        .add_attribute("membership_issuer_addr", data.membership_issuer_addr))
}

pub fn update_ask_fee_percentage_of_membership(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateAskFeePercentageOfMembershipMsg,
) -> Result<Response, ContractError> {
    let membership_issuer_addr_ref = &deps
        .api
        .addr_validate(data.membership_issuer_addr.as_str())
        .unwrap();
    if info.sender != USERS.load(deps.storage, membership_issuer_addr_ref)?.addr {
        return Err(ContractError::OnlyMembershipIssuerCanUpdateItsAskFeePercentageOfMembership {});
    }

    USERS.update(deps.storage, membership_issuer_addr_ref, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            let updated_user = User {
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                issued_membership: user.issued_membership,
                trading_fee_percentage_of_membership: user.trading_fee_percentage_of_membership,
                ask_fee_percentage_of_membership: Some(data.ask_fee_percentage_of_membership),
                ask_fee_to_thread_creator_percentage_of_membership: user
                    .ask_fee_to_thread_creator_percentage_of_membership,
                reply_fee_percentage_of_membership: user.reply_fee_percentage_of_membership,
                membership_trading_fee_share_config: user.membership_trading_fee_share_config,
                thread_fee_share_config: user.thread_fee_share_config,
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_ask_fee_percentage_of_membership")
        .add_attribute("membership_issuer_addr", data.membership_issuer_addr))
}

pub fn update_ask_fee_to_thread_creator_percentage_of_membership(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateAskFeeToThreadCreatorPercentageOfMembershipMsg,
) -> Result<Response, ContractError> {
    let membership_issuer_addr_ref = &deps
        .api
        .addr_validate(data.membership_issuer_addr.as_str())
        .unwrap();
    if info.sender != USERS.load(deps.storage, membership_issuer_addr_ref)?.addr {
        return Err(
            ContractError::OnlyMembershipIssuerCanUpdateItsAskFeeToCreatorPercentageOfMembership {},
        );
    }

    USERS.update(deps.storage, membership_issuer_addr_ref, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            let updated_user = User {
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                issued_membership: user.issued_membership,
                trading_fee_percentage_of_membership: user.trading_fee_percentage_of_membership,
                ask_fee_percentage_of_membership: user.ask_fee_percentage_of_membership,
                ask_fee_to_thread_creator_percentage_of_membership: Some(
                    data.ask_fee_to_thread_creator_percentage_of_membership,
                ),
                reply_fee_percentage_of_membership: user.reply_fee_percentage_of_membership,
                membership_trading_fee_share_config: user.membership_trading_fee_share_config,
                thread_fee_share_config: user.thread_fee_share_config,
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute(
            "action",
            "update_ask_fee_to_thread_creator_percentage_of_membership",
        )
        .add_attribute("membership_issuer_addr", data.membership_issuer_addr))
}

pub fn update_reply_fee_percentage_of_membership(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateReplyFeePercentageOfMembershipMsg,
) -> Result<Response, ContractError> {
    let membership_issuer_addr_ref = &deps
        .api
        .addr_validate(data.membership_issuer_addr.as_str())
        .unwrap();
    if info.sender != USERS.load(deps.storage, membership_issuer_addr_ref)?.addr {
        return Err(
            ContractError::OnlyMembershipIssuerCanUpdateItsReplyFeePercentageOfMembership {},
        );
    }

    USERS.update(deps.storage, membership_issuer_addr_ref, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            let updated_user = User {
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                issued_membership: user.issued_membership,
                trading_fee_percentage_of_membership: user.trading_fee_percentage_of_membership,
                ask_fee_percentage_of_membership: user.ask_fee_percentage_of_membership,
                ask_fee_to_thread_creator_percentage_of_membership: user
                    .ask_fee_to_thread_creator_percentage_of_membership,
                reply_fee_percentage_of_membership: Some(data.reply_fee_percentage_of_membership),
                membership_trading_fee_share_config: user.membership_trading_fee_share_config,
                thread_fee_share_config: user.thread_fee_share_config,
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_reply_fee_percentage_of_membership")
        .add_attribute("membership_issuer_addr", data.membership_issuer_addr))
}

pub fn update_membership_trading_fee_share_config(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateMembershipTradingFeeShareConfigMsg,
) -> Result<Response, ContractError> {
    let membership_issuer_addr_ref = &deps
        .api
        .addr_validate(data.membership_issuer_addr.as_str())
        .unwrap();

    if info.sender != USERS.load(deps.storage, membership_issuer_addr_ref)?.addr {
        return Err(ContractError::OnlyMembershipIssuerCanUpdateItsTradingFeeConfig {});
    }

    USERS.update(deps.storage, membership_issuer_addr_ref, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            let updated_user = User {
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                issued_membership: user.issued_membership,
                trading_fee_percentage_of_membership: user.trading_fee_percentage_of_membership,
                ask_fee_percentage_of_membership: user.ask_fee_percentage_of_membership,
                ask_fee_to_thread_creator_percentage_of_membership: user
                    .ask_fee_to_thread_creator_percentage_of_membership,
                reply_fee_percentage_of_membership: user.reply_fee_percentage_of_membership,
                membership_trading_fee_share_config: Some(data.membership_trading_fee_share_config),
                thread_fee_share_config: user.thread_fee_share_config,
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_trading_fee_config")
        .add_attribute("membership_issuer_addr", data.membership_issuer_addr))
}

pub fn update_thread_fee_share_config(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateThreadFeeShareConfigMsg,
) -> Result<Response, ContractError> {
    let membership_issuer_addr_ref = &deps
        .api
        .addr_validate(data.membership_issuer_addr.as_str())
        .unwrap();

    if info.sender != USERS.load(deps.storage, membership_issuer_addr_ref)?.addr {
        return Err(ContractError::OnlyMembershipIssuerCanUpdateItsThreadFeeConfig {});
    }

    USERS.update(deps.storage, membership_issuer_addr_ref, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            let updated_user = User {
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                issued_membership: user.issued_membership,
                trading_fee_percentage_of_membership: user.trading_fee_percentage_of_membership,
                ask_fee_percentage_of_membership: user.ask_fee_percentage_of_membership,
                ask_fee_to_thread_creator_percentage_of_membership: user
                    .ask_fee_to_thread_creator_percentage_of_membership,
                reply_fee_percentage_of_membership: user.reply_fee_percentage_of_membership,
                membership_trading_fee_share_config: user.membership_trading_fee_share_config,
                thread_fee_share_config: Some(data.thread_fee_share_config),
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_thread_fee_config")
        .add_attribute("membership_issuer_addr", data.membership_issuer_addr))
}
