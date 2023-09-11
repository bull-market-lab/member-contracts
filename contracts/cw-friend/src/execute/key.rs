use cosmwasm_std::{BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128};

use friend::{
    config::Config,
    msg::{
        BuyKeyMsg, QuerySimulateBuyKeyMsg, QuerySimulateSellKeyMsg, SellKeyMsg,
        SimulateBuyKeyResponse, SimulateSellKeyResponse,
    },
};

use crate::{
    state::{ALL_KEYS_HOLDERS, ALL_USERS_HOLDINGS},
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
    let key_issuer_addr_ref = &data.key_issuer_addr;

    let simulate_buy_key_response: SimulateBuyKeyResponse = deps.querier.query_wasm_smart(
        env.contract.address,
        &QuerySimulateBuyKeyMsg {
            key_issuer_addr: data.key_issuer_addr.clone(),
            amount: data.amount,
        },
    )?;

    let required_paid_amount = simulate_buy_key_response.price
        + simulate_buy_key_response.protocol_fee
        + simulate_buy_key_response.key_issuer_fee;
    if required_paid_amount > user_paid_amount {
        return Err(ContractError::InsufficientFunds {
            needed: required_paid_amount,
            available: user_paid_amount,
        });
    }

    let previous_amount = ALL_USERS_HOLDINGS
        .may_load(deps.storage, (sender_addr_ref, key_issuer_addr_ref))?
        .unwrap_or_default();
    ALL_USERS_HOLDINGS.save(
        deps.storage,
        (sender_addr_ref, key_issuer_addr_ref),
        &(previous_amount + data.amount),
    )?;
    ALL_KEYS_HOLDERS.save(
        deps.storage,
        (key_issuer_addr_ref, sender_addr_ref),
        &(previous_amount + data.amount),
    )?;

    let msgs_vec = vec![
        // Send key issuer fee to key issuer
        CosmosMsg::Bank(BankMsg::Send {
            to_address: data.key_issuer_addr.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: simulate_buy_key_response.key_issuer_fee,
            }],
        }),
        // Send protocol fee to fee collector
        CosmosMsg::Bank(BankMsg::Send {
            to_address: config.protocol_fee_collector_addr.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: simulate_buy_key_response.protocol_fee,
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
    let key_issuer_addr_ref = &data.key_issuer_addr;

    let simulate_sell_key_response: SimulateSellKeyResponse = deps.querier.query_wasm_smart(
        env.contract.address,
        &QuerySimulateSellKeyMsg {
            key_issuer_addr: data.key_issuer_addr.clone(),
            amount: data.amount,
        },
    )?;

    let required_paid_amount =
        simulate_sell_key_response.protocol_fee + simulate_sell_key_response.key_issuer_fee;
    if required_paid_amount > user_paid_amount {
        return Err(ContractError::InsufficientFunds {
            needed: required_paid_amount,
            available: user_paid_amount,
        });
    }

    let previous_amount = ALL_USERS_HOLDINGS
        .may_load(deps.storage, (sender_addr_ref, key_issuer_addr_ref))?
        .unwrap_or_default();
    if previous_amount < data.amount {
        return Err(ContractError::InsufficientKeysToSell {
            sell: data.amount,
            available: previous_amount,
        });
    }
    ALL_USERS_HOLDINGS.save(
        deps.storage,
        (sender_addr_ref, key_issuer_addr_ref),
        &(previous_amount - data.amount),
    )?;
    ALL_KEYS_HOLDERS.save(
        deps.storage,
        (key_issuer_addr_ref, sender_addr_ref),
        &(previous_amount - data.amount),
    )?;

    let msgs_vec = vec![
        // Send sell amount to seller
        CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: simulate_sell_key_response.price,
            }],
        }),
        // Send key issuer fee to key issuer
        CosmosMsg::Bank(BankMsg::Send {
            to_address: data.key_issuer_addr.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: simulate_sell_key_response.key_issuer_fee,
            }],
        }),
        // Send protocol fee to fee collector
        CosmosMsg::Bank(BankMsg::Send {
            to_address: config.protocol_fee_collector_addr.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: simulate_sell_key_response.protocol_fee,
            }],
        }),
    ];

    Ok(Response::new().add_messages(msgs_vec))
}
