use cosmwasm_std::{BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128};

use thread::{
    config::Config,
    msg::{
        BuyKeyMsg, CostToBuyKeyResponse, CostToSellKeyResponse, QueryCostToBuyKeyMsg,
        QueryCostToSellKeyMsg, QueryMsg, SellKeyMsg,
    },
};

use crate::{
    state::{ALL_KEYS_HOLDERS, ALL_USERS_HOLDINGS, KEY_SUPPLY},
    ContractError,
};

pub fn buy_key(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    data: BuyKeyMsg,
    config: Config,
    user_paid_amount: Uint128,
) -> Result<Response, ContractError> {
    let sender_addr_ref = &info.sender;
    let key_issuer_addr_ref = &deps
        .api
        .addr_validate(data.key_issuer_addr.as_str())
        .unwrap();

    KEY_SUPPLY.update(deps.storage, key_issuer_addr_ref, |supply| match supply {
        None => Err(ContractError::UserNotExist {}),
        Some(supply) => Ok(supply + data.amount),
    })?;

    let cost_to_buy_key_response: CostToBuyKeyResponse = deps.querier.query_wasm_smart(
        env.contract.address,
        &QueryMsg::QueryCostToBuyKey(QueryCostToBuyKeyMsg {
            key_issuer_addr: data.key_issuer_addr.clone(),
            amount: data.amount,
        }),
    )?;

    if cost_to_buy_key_response.total_needed_from_user > user_paid_amount {
        return Err(ContractError::InsufficientFundsToPayDuringBuy {
            needed: cost_to_buy_key_response.total_needed_from_user,
            available: user_paid_amount,
        });
    }

    let user_previous_hold_amount = ALL_USERS_HOLDINGS
        .may_load(deps.storage, (sender_addr_ref, key_issuer_addr_ref))?
        .unwrap_or_default();
    ALL_USERS_HOLDINGS.save(
        deps.storage,
        (sender_addr_ref, key_issuer_addr_ref),
        &(user_previous_hold_amount + data.amount),
    )?;
    ALL_KEYS_HOLDERS.save(
        deps.storage,
        (key_issuer_addr_ref, sender_addr_ref),
        &(user_previous_hold_amount + data.amount),
    )?;

    let msgs_vec = vec![
        // Send key issuer fee to key issuer
        CosmosMsg::Bank(BankMsg::Send {
            to_address: data.key_issuer_addr.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: cost_to_buy_key_response.key_issuer_fee,
            }],
        }),
        // TODO: P0: distribute key holder fee to all key holders
        // Send protocol fee to fee collector
        CosmosMsg::Bank(BankMsg::Send {
            to_address: config.protocol_fee_collector_addr.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: cost_to_buy_key_response.protocol_fee,
            }],
        }),
    ];

    Ok(Response::new().add_messages(msgs_vec))
}

pub fn sell_key(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    data: SellKeyMsg,
    config: Config,
    user_paid_amount: Uint128,
) -> Result<Response, ContractError> {
    let sender_addr_ref = &info.sender;
    let key_issuer_addr_ref = &deps
        .api
        .addr_validate(data.key_issuer_addr.as_str())
        .unwrap();

    let total_supply = KEY_SUPPLY.load(deps.storage, key_issuer_addr_ref)?;
    if total_supply < data.amount {
        return Err(ContractError::CannotSellLastKey {
            sell: data.amount,
            total_supply,
        });
    }

    KEY_SUPPLY.update(deps.storage, key_issuer_addr_ref, |supply| match supply {
        None => Err(ContractError::UserNotExist {}),
        Some(supply) => Ok(supply - data.amount),
    })?;

    let user_previous_hold_amount = ALL_USERS_HOLDINGS
        .may_load(deps.storage, (sender_addr_ref, key_issuer_addr_ref))?
        .unwrap_or_default();
    if user_previous_hold_amount < data.amount {
        return Err(ContractError::InsufficientKeysToSell {
            sell: data.amount,
            available: user_previous_hold_amount,
        });
    }

    let cost_to_sell_key_response: CostToSellKeyResponse = deps.querier.query_wasm_smart(
        env.contract.address,
        &QueryMsg::QueryCostToSellKey(QueryCostToSellKeyMsg {
            key_issuer_addr: data.key_issuer_addr.clone(),
            amount: data.amount,
        }),
    )?;

    if cost_to_sell_key_response.total_needed_from_user > user_paid_amount {
        return Err(ContractError::InsufficientFundsToPayDuringSell {
            needed: cost_to_sell_key_response.total_needed_from_user,
            available: user_paid_amount,
        });
    }

    ALL_USERS_HOLDINGS.save(
        deps.storage,
        (sender_addr_ref, key_issuer_addr_ref),
        &(user_previous_hold_amount - data.amount),
    )?;
    ALL_KEYS_HOLDERS.save(
        deps.storage,
        (key_issuer_addr_ref, sender_addr_ref),
        &(user_previous_hold_amount - data.amount),
    )?;

    let msgs_vec = vec![
        // Send sell amount to seller
        CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: cost_to_sell_key_response.price,
            }],
        }),
        // Send key issuer fee to key issuer
        CosmosMsg::Bank(BankMsg::Send {
            to_address: data.key_issuer_addr.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: cost_to_sell_key_response.key_issuer_fee,
            }],
        }),
        // TODO: P0: distribute key holder fee to all key holders
        // Send protocol fee to fee collector
        CosmosMsg::Bank(BankMsg::Send {
            to_address: config.protocol_fee_collector_addr.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: cost_to_sell_key_response.protocol_fee,
            }],
        }),
    ];

    Ok(Response::new().add_messages(msgs_vec))
}
