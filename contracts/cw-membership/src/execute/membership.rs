use cosmwasm_std::{
    BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128, Uint64,
};
use membership::{
    config::Config,
    msg::{
        BuyMembershipMsg, CostToBuyMembershipResponse, CostToSellMembershipResponse,
        QueryCostToBuyMembershipMsg, QueryCostToSellMembershipMsg, QueryMsg, SellMembershipMsg,
    },
};

use crate::{
    state::{ALL_MEMBERSHIPS_MEMBERS, ALL_USERS_MEMBERSHIPS, USERS},
    util::user::get_cosmos_msgs_to_distribute_fee_to_all_members,
    ContractError,
};

pub fn buy_membership(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    data: BuyMembershipMsg,
    config: Config,
    user_paid_amount: Uint128,
) -> Result<Response, ContractError> {
    let sender_user_id = USERS().load(deps.storage, &info.sender)?.id.u64();

    let membership_issuer_user_id = data.membership_issuer_user_id.u64();
    let membership_issuer = USERS()
        .idx
        .id
        .item(deps.storage, membership_issuer_user_id)?
        .unwrap()
        .1;
    let membership_issuer_addr_ref = &membership_issuer.addr;

    let cost_to_buy_membership_response: CostToBuyMembershipResponse =
        deps.querier.query_wasm_smart(
            env.contract.address,
            &QueryMsg::QueryCostToBuyMembership(QueryCostToBuyMembershipMsg {
                membership_issuer_user_id: Uint64::from(membership_issuer_user_id),
                amount: data.amount,
            }),
        )?;

    if cost_to_buy_membership_response.total_needed_from_user > user_paid_amount {
        return Err(ContractError::InsufficientFundsToPayDuringBuy {
            needed: cost_to_buy_membership_response.total_needed_from_user,
            available: user_paid_amount,
        });
    }

    let user_previous_hold_amount = ALL_USERS_MEMBERSHIPS
        .may_load(deps.storage, (sender_user_id, membership_issuer_user_id))?
        .unwrap_or(Uint128::zero());
    let user_new_hold_amount = user_previous_hold_amount + data.amount;

    let total_supply = membership_issuer
        .membership_issued_by_me
        .unwrap()
        .membership_supply;

    // Split and send member fee to all members, this should include the sender's new buy amount
    // But sender should not receive the part of fee for the new memberships it bought
    // e.g. previously user 1 holds 5 memberships, user 2 holds 5 memberships
    // User 1 buys 5 memberships now, but user 1 and user 2 splits the fee 50 / 50
    // Because user 1 had 5 memberships before just like user 2
    let mut msgs_vec = get_cosmos_msgs_to_distribute_fee_to_all_members(
        deps.storage,
        config.fee_denom.clone(),
        cost_to_buy_membership_response.all_members_fee,
        membership_issuer_user_id,
        total_supply,
    );

    msgs_vec.push(
        // Send membership issuer fee to membership issuer
        CosmosMsg::Bank(BankMsg::Send {
            to_address: membership_issuer_addr_ref.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: cost_to_buy_membership_response.issuer_fee,
            }],
        }),
    );
    msgs_vec.push(
        // Send protocol fee to fee collector
        CosmosMsg::Bank(BankMsg::Send {
            to_address: config.protocol_fee_collector_addr.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: cost_to_buy_membership_response.protocol_fee,
            }],
        }),
    );

    // Update membership supply
    USERS().update(
        deps.storage,
        membership_issuer_addr_ref,
        |user| match user {
            None => Err(ContractError::UserNotExist {}),
            Some(mut user) => {
                user.membership_issued_by_me
                    .as_mut()
                    .unwrap()
                    .membership_supply += data.amount;
                Ok(user)
            }
        },
    )?;

    if user_previous_hold_amount == Uint128::zero() {
        USERS().update(
            deps.storage,
            membership_issuer_addr_ref,
            |user| match user {
                None => Err(ContractError::UserNotExist {}),
                Some(mut user) => {
                    user.membership_issued_by_me.as_mut().unwrap().member_count += Uint128::one();
                    Ok(user)
                }
            },
        )?;
        USERS().update(deps.storage, &info.sender, |user| match user {
            None => Err(ContractError::UserNotExist {}),
            Some(mut user) => {
                user.user_member_count += Uint128::one();
                Ok(user)
            }
        })?;
    }

    ALL_USERS_MEMBERSHIPS.save(
        deps.storage,
        (sender_user_id, membership_issuer_user_id),
        &user_new_hold_amount,
    )?;
    ALL_MEMBERSHIPS_MEMBERS.save(
        deps.storage,
        (membership_issuer_user_id, sender_user_id),
        &user_new_hold_amount,
    )?;

    Ok(Response::new().add_messages(msgs_vec))
}

pub fn sell_membership(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    data: SellMembershipMsg,
    config: Config,
    user_paid_amount: Uint128,
) -> Result<Response, ContractError> {
    let sender_user_id = USERS().load(deps.storage, &info.sender)?.id.u64();

    let membership_issuer_user_id = data.membership_issuer_user_id.u64();
    let membership_issuer = USERS()
        .idx
        .id
        .item(deps.storage, membership_issuer_user_id)?
        .unwrap()
        .1;
    let membership_issuer_addr_ref = &membership_issuer.addr;

    let total_supply = membership_issuer
        .membership_issued_by_me
        .unwrap()
        .membership_supply;
    if total_supply <= data.amount {
        return Err(ContractError::CannotSellLastMembership {
            sell: data.amount,
            total_supply,
        });
    }

    let user_previous_hold_amount = ALL_USERS_MEMBERSHIPS
        .may_load(deps.storage, (sender_user_id, membership_issuer_user_id))?
        .unwrap_or(Uint128::zero());
    if user_previous_hold_amount < data.amount {
        return Err(ContractError::InsufficientMembershipsToSell {
            sell: data.amount,
            available: user_previous_hold_amount,
        });
    }
    let user_new_hold_amount = user_previous_hold_amount - data.amount;

    let cost_to_sell_membership_response: CostToSellMembershipResponse =
        deps.querier.query_wasm_smart(
            env.contract.address,
            &QueryMsg::QueryCostToSellMembership(QueryCostToSellMembershipMsg {
                membership_issuer_user_id: Uint64::from(membership_issuer_user_id),
                amount: data.amount,
            }),
        )?;

    if cost_to_sell_membership_response.total_needed_from_user > user_paid_amount {
        return Err(ContractError::InsufficientFundsToPayDuringSell {
            needed: cost_to_sell_membership_response.total_needed_from_user,
            available: user_paid_amount,
        });
    }

    // Update membership supply
    USERS().update(
        deps.storage,
        membership_issuer_addr_ref,
        |user| match user {
            None => Err(ContractError::UserNotExist {}),
            Some(mut user) => {
                user.membership_issued_by_me
                    .as_mut()
                    .unwrap()
                    .membership_supply -= data.amount;
                Ok(user)
            }
        },
    )?;

    if user_new_hold_amount == Uint128::zero() {
        USERS().update(
            deps.storage,
            membership_issuer_addr_ref,
            |user| match user {
                None => Err(ContractError::UserNotExist {}),
                Some(mut user) => {
                    user.membership_issued_by_me.as_mut().unwrap().member_count -= Uint128::one();
                    Ok(user)
                }
            },
        )?;
        USERS().update(deps.storage, &info.sender, |user| match user {
            None => Err(ContractError::UserNotExist {}),
            Some(mut user) => {
                user.user_member_count -= Uint128::one();
                Ok(user)
            }
        })?;
    }

    ALL_USERS_MEMBERSHIPS.save(
        deps.storage,
        (sender_user_id, membership_issuer_user_id),
        &user_new_hold_amount,
    )?;
    ALL_MEMBERSHIPS_MEMBERS.save(
        deps.storage,
        (membership_issuer_user_id, sender_user_id),
        &user_new_hold_amount,
    )?;

    // Split and send member fee to all members, this should exclude the sender's sell amount
    // But sender should not receive the part of fee for the memberships it sells
    // e.g. previously user 1 holds 5 memberships, user 2 holds 10 memberships
    // User 2 sells 5 memberships now, user 1 and user 2 splits the fee 50 / 50
    // Because user 2 only has 5 memberships now just like user 1
    let mut msgs_vec = get_cosmos_msgs_to_distribute_fee_to_all_members(
        deps.storage,
        config.fee_denom.clone(),
        cost_to_sell_membership_response.all_members_fee,
        membership_issuer_user_id,
        total_supply - data.amount,
    );

    msgs_vec.push(
        // Send membership issuer fee to membership issuer
        CosmosMsg::Bank(BankMsg::Send {
            to_address: membership_issuer_addr_ref.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: cost_to_sell_membership_response.issuer_fee,
            }],
        }),
    );

    msgs_vec.push(
        // Send protocol fee to fee collector
        CosmosMsg::Bank(BankMsg::Send {
            to_address: config.protocol_fee_collector_addr.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: cost_to_sell_membership_response.protocol_fee,
            }],
        }),
    );
    msgs_vec.push(
        // Send sell amount to seller
        CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: cost_to_sell_membership_response.price,
            }],
        }),
    );

    Ok(Response::new().add_messages(msgs_vec))
}
