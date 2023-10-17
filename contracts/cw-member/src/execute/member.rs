use cosmwasm_std::{
    to_binary, BankMsg, Coin, CosmosMsg, Decimal, DepsMut, MessageInfo, Response, Uint128, Uint64,
    WasmMsg,
};
use distribution::msg::{
    DistributeMsg, ExecuteMsg, SetupDistributionForNewMemberMsg, UpdateUserPendingRewardMsg,
};
use member::{
    config::Config,
    msg::{
        BuyMembershipMsg, CostToBuyMembershipResponse, CostToSellMembershipResponse,
        QueryCostToBuyMembershipMsg, QueryCostToSellMembershipMsg, SellMembershipMsg,
    },
};

use crate::{
    query::cost::{query_cost_to_buy_membership, query_cost_to_sell_membership},
    state::{ALL_MEMBERSHIPS_MEMBERS, ALL_USERS, ALL_USERS_MEMBERSHIPS},
    ContractError,
};

pub fn buy_membership(
    deps: DepsMut,
    info: MessageInfo,
    data: BuyMembershipMsg,
    config: Config,
    user_paid_amount: Uint128,
    fee_denom: String,
) -> Result<Response, ContractError> {
    let buyer_user_id = ALL_USERS().load(deps.storage, &info.sender)?.id.u64();

    let membership_issuer_user_id = data.membership_issuer_user_id.u64();
    let membership_issuer = ALL_USERS()
        .idx
        .id
        .item(deps.storage, membership_issuer_user_id)?
        .unwrap()
        .1;
    let membership_issuer_addr_ref = &membership_issuer.addr;

    let cost_to_buy_membership_response: CostToBuyMembershipResponse =
        query_cost_to_buy_membership(
            deps.as_ref(),
            QueryCostToBuyMembershipMsg {
                membership_issuer_user_id: Uint64::from(membership_issuer_user_id),
                amount: data.amount,
            },
            config.clone(),
        )?;

    if cost_to_buy_membership_response.total_needed_from_user > user_paid_amount {
        return Err(ContractError::InsufficientFundsToPayDuringBuy {
            needed: cost_to_buy_membership_response.total_needed_from_user,
            available: user_paid_amount,
        });
    }

    let buyer_previous_hold_amount = ALL_USERS_MEMBERSHIPS
        .may_load(deps.storage, (buyer_user_id, membership_issuer_user_id))?
        .unwrap_or(Uint128::zero());
    let buyer_new_hold_amount = buyer_previous_hold_amount + data.amount;

    let previous_total_supply = membership_issuer
        .membership_issued_by_me
        .unwrap()
        .membership_supply;

    let mut msgs_vec = vec![
        // Send all member fee to distribution contract
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config
                .distribution_contract_addr
                .clone()
                .unwrap()
                .to_string(),
            msg: to_binary(&ExecuteMsg::Distribute(DistributeMsg {
                membership_issuer_user_id: Uint64::from(membership_issuer_user_id),
                index_increment: Decimal::from_ratio(
                    cost_to_buy_membership_response.all_members_fee,
                    previous_total_supply,
                ),
            }))?,
            funds: vec![Coin {
                denom: fee_denom.clone(),
                amount: cost_to_buy_membership_response.all_members_fee,
            }],
        }),
        // Send membership issuer fee to membership issuer
        CosmosMsg::Bank(BankMsg::Send {
            to_address: membership_issuer_addr_ref.to_string(),
            amount: vec![Coin {
                denom: fee_denom.clone(),
                amount: cost_to_buy_membership_response.issuer_fee,
            }],
        }),
        // Send protocol fee to fee collector
        CosmosMsg::Bank(BankMsg::Send {
            to_address: config.protocol_fee_collector_addr.to_string(),
            amount: vec![Coin {
                denom: fee_denom,
                amount: cost_to_buy_membership_response.protocol_fee,
            }],
        }),
    ];

    // Update membership supply
    ALL_USERS().update(
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

    if buyer_previous_hold_amount == Uint128::zero() {
        ALL_USERS().update(
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
        ALL_USERS().update(deps.storage, &info.sender, |user| match user {
            None => Err(ContractError::UserNotExist {}),
            Some(mut user) => {
                user.user_member_count += Uint128::one();
                Ok(user)
            }
        })?;
    }

    ALL_USERS_MEMBERSHIPS.save(
        deps.storage,
        (buyer_user_id, membership_issuer_user_id),
        &buyer_new_hold_amount,
    )?;
    ALL_MEMBERSHIPS_MEMBERS.save(
        deps.storage,
        (membership_issuer_user_id, buyer_user_id),
        &buyer_new_hold_amount,
    )?;

    let distribution_contract_addr = config.distribution_contract_addr.unwrap().to_string();

    if buyer_previous_hold_amount == Uint128::zero() {
        msgs_vec.push(
            // Setup distribution for first time buyer
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: distribution_contract_addr.clone(),
                msg: to_binary(&ExecuteMsg::SetupDistributionForNewMember(
                    SetupDistributionForNewMemberMsg {
                        user_id: Uint64::from(buyer_user_id),
                        membership_issuer_user_id: Uint64::from(membership_issuer_user_id),
                    },
                ))?,
                funds: vec![],
            }),
        );
    }

    msgs_vec.push(
        // Update buyer's pending reward in distribution contract
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: distribution_contract_addr,
            msg: to_binary(&ExecuteMsg::UpdateUserPendingReward(
                UpdateUserPendingRewardMsg {
                    user_id: Uint64::from(buyer_user_id),
                    membership_issuer_user_id: Uint64::from(membership_issuer_user_id),
                    user_previous_amount: buyer_previous_hold_amount,
                },
            ))?,
            funds: vec![],
        }),
    );

    Ok(Response::new().add_messages(msgs_vec))
}

pub fn sell_membership(
    deps: DepsMut,
    info: MessageInfo,
    data: SellMembershipMsg,
    config: Config,
    user_paid_amount: Uint128,
    fee_denom: String,
) -> Result<Response, ContractError> {
    let seller_user_id = ALL_USERS().load(deps.storage, &info.sender)?.id.u64();

    let membership_issuer_user_id = data.membership_issuer_user_id.u64();
    let membership_issuer = ALL_USERS()
        .idx
        .id
        .item(deps.storage, membership_issuer_user_id)?
        .unwrap()
        .1;
    let membership_issuer_addr_ref = &membership_issuer.addr;

    let previous_total_supply = membership_issuer
        .membership_issued_by_me
        .unwrap()
        .membership_supply;
    if previous_total_supply <= data.amount {
        return Err(ContractError::CannotSellLastMembership {
            sell: data.amount,
            total_supply: previous_total_supply,
        });
    }

    let seller_previous_hold_amount = ALL_USERS_MEMBERSHIPS
        .may_load(deps.storage, (seller_user_id, membership_issuer_user_id))?
        .unwrap_or(Uint128::zero());
    if seller_previous_hold_amount < data.amount {
        return Err(ContractError::InsufficientMembershipsToSell {
            sell: data.amount,
            available: seller_previous_hold_amount,
        });
    }
    let seller_new_hold_amount = seller_previous_hold_amount - data.amount;

    let cost_to_sell_membership_response: CostToSellMembershipResponse =
        query_cost_to_sell_membership(
            deps.as_ref(),
            QueryCostToSellMembershipMsg {
                membership_issuer_user_id: Uint64::from(membership_issuer_user_id),
                amount: data.amount,
            },
            config.clone(),
        )?;

    if cost_to_sell_membership_response.total_needed_from_user > user_paid_amount {
        return Err(ContractError::InsufficientFundsToPayDuringSell {
            needed: cost_to_sell_membership_response.total_needed_from_user,
            available: user_paid_amount,
        });
    }

    // Update membership supply
    ALL_USERS().update(
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

    if seller_new_hold_amount == Uint128::zero() {
        ALL_USERS().update(
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
        ALL_USERS().update(deps.storage, &info.sender, |user| match user {
            None => Err(ContractError::UserNotExist {}),
            Some(mut user) => {
                user.user_member_count -= Uint128::one();
                Ok(user)
            }
        })?;
    }

    // TODO: P0: should we delete the key if the value becomes zero?
    ALL_USERS_MEMBERSHIPS.save(
        deps.storage,
        (seller_user_id, membership_issuer_user_id),
        &seller_new_hold_amount,
    )?;
    ALL_MEMBERSHIPS_MEMBERS.save(
        deps.storage,
        (membership_issuer_user_id, seller_user_id),
        &seller_new_hold_amount,
    )?;

    let mut msgs_vec = vec![
        // Send all member fee to distribution contract
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config
                .distribution_contract_addr
                .clone()
                .unwrap()
                .to_string(),
            msg: to_binary(&ExecuteMsg::Distribute(DistributeMsg {
                membership_issuer_user_id: Uint64::from(membership_issuer_user_id),
                index_increment: Decimal::from_ratio(
                    cost_to_sell_membership_response.all_members_fee,
                    previous_total_supply,
                ),
            }))?,
            funds: vec![Coin {
                denom: fee_denom.clone(),
                amount: cost_to_sell_membership_response.all_members_fee,
            }],
        }),
        // Send membership issuer fee to membership issuer
        CosmosMsg::Bank(BankMsg::Send {
            to_address: membership_issuer_addr_ref.to_string(),
            amount: vec![Coin {
                denom: fee_denom.clone(),
                amount: cost_to_sell_membership_response.issuer_fee,
            }],
        }),
        // Send protocol fee to fee collector
        CosmosMsg::Bank(BankMsg::Send {
            to_address: config.protocol_fee_collector_addr.to_string(),
            amount: vec![Coin {
                denom: fee_denom.clone(),
                amount: cost_to_sell_membership_response.protocol_fee,
            }],
        }),
        // Send sell amount to seller
        CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: fee_denom,
                amount: cost_to_sell_membership_response.price,
            }],
        }),
    ];

    msgs_vec.push(
        // Update seller's pending reward in distribution contract
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.distribution_contract_addr.unwrap().to_string(),
            msg: to_binary(&ExecuteMsg::UpdateUserPendingReward(
                UpdateUserPendingRewardMsg {
                    user_id: Uint64::from(seller_user_id),
                    membership_issuer_user_id: Uint64::from(membership_issuer_user_id),
                    user_previous_amount: seller_previous_hold_amount,
                },
            ))?,
            funds: vec![],
        }),
    );

    Ok(Response::new().add_messages(msgs_vec))
}
