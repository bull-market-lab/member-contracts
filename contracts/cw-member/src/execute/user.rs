use cosmwasm_std::{
    to_binary, CosmosMsg, DepsMut, MessageInfo, Response, Uint128, Uint64, WasmMsg,
};

use distribution::msg::{
    ExecuteMsg, SetupDistributionForNewMemberMsg, SetupDistributionForNewMembershipMsg,
};
use member::{
    config::{Config, FeeConfig},
    msg::{EnableMembershipMsg, LinkSocialMediaMsg, UpdateUserConfigMsg},
    user::{MembershipIssuedByMe, User},
};
use shared::fee_share_config::FeeShareConfig;

use crate::{
    state::{ALL_MEMBERSHIPS_MEMBERS, ALL_USERS, ALL_USERS_MEMBERSHIPS, NEXT_USER_ID},
    util::fee_share::assert_user_fee_share_sum_to_100,
    ContractError,
};

pub fn register(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let sender_addr_ref = &info.sender;
    if ALL_USERS()
        .may_load(deps.storage, sender_addr_ref)?
        .is_some()
    {
        return Err(ContractError::AddressAlreadyRegistered {});
    }

    let user_id = NEXT_USER_ID.load(deps.storage)?;

    if ALL_USERS()
        .idx
        .id
        .item(deps.storage, user_id.u64())?
        .is_some()
    {
        return Err(ContractError::UserIDAlreadyUsedDuringRegistration {});
    }

    ALL_USERS().save(
        deps.storage,
        sender_addr_ref,
        &User {
            id: user_id,
            addr: info.sender.clone(),
            social_media_handle: None,
            membership_issued_by_me: None,
            // TODO: P1: support custom fee config during registration and update
            fee_config: None,
            fee_share_config: None,
            user_member_count: Uint128::zero(),
        },
    )?;

    NEXT_USER_ID.save(deps.storage, &(user_id + Uint64::one()))?;

    Ok(Response::new()
        .add_attribute("action", "register_user")
        .add_attribute("user_id", user_id)
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

    let user = ALL_USERS()
        .idx
        .id
        .item(deps.storage, data.user_id.u64())?
        .unwrap()
        .1;

    ALL_USERS().update(deps.storage, &user.addr, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            // User should not have linked a social media handle yet
            if user.social_media_handle.is_some() {
                return Err(ContractError::UserAlreadyLinkedSocialMedia {});
            }
            let updated_user = User {
                id: user.id,
                addr: user.addr,
                social_media_handle: Some(data.social_media_handle.clone()),
                membership_issued_by_me: user.membership_issued_by_me,
                fee_config: user.fee_config,
                fee_share_config: user.fee_share_config,
                user_member_count: user.user_member_count,
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "link_social_media")
        .add_attribute("user_id", user.id)
        .add_attribute("user_addr", user.addr)
        .add_attribute("social_media_handle", data.social_media_handle))
}

pub fn enable_membership(
    deps: DepsMut,
    info: MessageInfo,
    data: EnableMembershipMsg,
    config: Config,
) -> Result<Response, ContractError> {
    if info.sender != config.registration_admin_addr {
        return Err(ContractError::OnlyRegistrationAdminCanEnableMembershipOnBehalfOfUser {});
    }

    let user = ALL_USERS()
        .idx
        .id
        .item(deps.storage, data.user_id.u64())?
        .unwrap()
        .1;
    let user_id = user.id.u64();

    // User should not have a Membership yet
    if user.membership_issued_by_me.is_some() {
        return Err(ContractError::UserAlreadyRegisteredMembership {});
    }

    // User must already have a social media handle
    if user.social_media_handle.is_none() {
        return Err(ContractError::UserCannotRegisterMembershipBeforeLinkingSocialMedia {});
    }

    ALL_USERS().update(deps.storage, &user.addr, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            let updated_user = User {
                id: user.id,
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                membership_issued_by_me: Some(MembershipIssuedByMe {
                    membership_supply: Uint128::one(),
                    member_count: Uint128::one(),
                }),
                fee_config: user.fee_config,
                fee_share_config: user.fee_share_config,
                // User is a new member of itself
                user_member_count: user.user_member_count + Uint128::one(),
            };
            Ok(updated_user)
        }
    })?;

    ALL_USERS_MEMBERSHIPS.update(deps.storage, (user_id, user_id), |existing_holding| {
        match existing_holding {
            // User should not hold its own Membership because it has never registered a Membership before
            Some(_) => Err(ContractError::UserAlreadyRegisteredMembership {}),
            // User should hold 1 new Membership which is issued by itself now
            None => Ok(Uint128::one()),
        }
    })?;

    ALL_MEMBERSHIPS_MEMBERS.update(deps.storage, (user_id, user_id), |existing_holder| {
        match existing_holder {
            // User's Membership should not have any holder before because it has never registered a Membership before
            Some(_) => Err(ContractError::UserAlreadyRegisteredMembership {}),
            // User's Membership should have 1 holder now which is itself
            None => Ok(Uint128::one()),
        }
    })?;

    let distribution_contract_addr = config.distribution_contract_addr.unwrap().to_string();

    let msgs_vec = vec![
        // Setup distribution for new membership program
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: distribution_contract_addr.clone(),
            msg: to_binary(&ExecuteMsg::SetupDistributionForNewMembership(
                SetupDistributionForNewMembershipMsg {
                    membership_issuer_user_id: Uint64::from(user_id),
                },
            ))?,
            funds: vec![],
        }),
        // Setup distribution for new member, since user is its own first member
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: distribution_contract_addr,
            msg: to_binary(&ExecuteMsg::SetupDistributionForNewMember(
                SetupDistributionForNewMemberMsg {
                    membership_issuer_user_id: Uint64::from(user_id),
                    user_id: Uint64::from(user_id),
                },
            ))?,
            funds: vec![],
        }),
    ];

    Ok(Response::new()
        .add_messages(msgs_vec)
        .add_attribute("action", "enable_membership")
        .add_attribute("user_id", user.id)
        .add_attribute("user_addr", user.addr))
}

pub fn update_user_config(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateUserConfigMsg,
) -> Result<Response, ContractError> {
    let user_id = data.user_id.u64();
    let user = ALL_USERS().idx.id.item(deps.storage, user_id)?.unwrap().1;
    let user_addr_ref = &user.addr;

    if info.sender != *user_addr_ref {
        return Err(
            ContractError::OnlyMembershipIssuerCanUpdateItsTradingFeePercentageOfMembership {},
        );
    }

    ALL_USERS().update(deps.storage, user_addr_ref, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            let updated_user = User {
                id: user.id,
                addr: user.addr,
                social_media_handle: user.social_media_handle,
                membership_issued_by_me: user.membership_issued_by_me,
                fee_config: match data.trading_fee_percentage_of_membership {
                    None => user.fee_config,
                    Some(data) => Some(FeeConfig {
                        trading_fee_percentage_of_membership: data,
                        fee_denom: user.fee_config.unwrap().fee_denom,
                    }),
                },
                fee_share_config: if data.share_to_issuer_percentage.is_none() {
                    user.fee_share_config
                } else {
                    Some(FeeShareConfig {
                        share_to_issuer_percentage: data.share_to_issuer_percentage.unwrap(),
                        share_to_all_members_percentage: data
                            .share_to_all_members_percentage
                            .unwrap(),
                    })
                },
                user_member_count: user.user_member_count,
            };
            Ok(updated_user)
        }
    })?;

    assert_user_fee_share_sum_to_100(deps.as_ref(), user_id)?;

    Ok(Response::new()
        .add_attribute("action", "update_user_config")
        .add_attribute("user_id", user.id)
        .add_attribute("membership_issuer_addr", user_addr_ref.to_string()))
}
