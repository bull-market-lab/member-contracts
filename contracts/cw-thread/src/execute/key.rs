use cosmwasm_std::{BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128};

use thread::{
    config::Config,
    key::Key,
    msg::{
        BuyKeyMsg, QueryMsg, QuerySimulateBuyKeyMsg, QuerySimulateSellKeyMsg, SellKeyMsg,
        SimulateBuyKeyResponse, SimulateSellKeyResponse,
    },
    user::User,
};

use crate::{
    state::{ALL_KEYS_HOLDERS, ALL_USERS_HOLDINGS, USERS},
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

    USERS.update(
        deps.storage,
        key_issuer_addr_ref,
        |key_issuer| match key_issuer {
            None => Err(ContractError::UserNotExist {}),
            Some(key_issuer) => {
                let updated_key_issuer = User {
                    addr: key_issuer.addr,
                    social_media_handle: key_issuer.social_media_handle,
                    issued_key: Some(Key {
                        supply: key_issuer.issued_key.clone().unwrap().supply + data.amount,
                        key_trading_fee_config: key_issuer
                            .issued_key
                            .clone()
                            .unwrap()
                            .key_trading_fee_config,
                        qa_fee_config: key_issuer.issued_key.unwrap().qa_fee_config,
                    }),
                };
                Ok(updated_key_issuer)
            }
        },
    )?;

    let simulate_buy_key_response: SimulateBuyKeyResponse = deps.querier.query_wasm_smart(
        env.contract.address,
        &QueryMsg::QuerySimulateBuyKey(QuerySimulateBuyKeyMsg {
            key_issuer_addr: data.key_issuer_addr.clone(),
            amount: data.amount,
        }),
    )?;

    if simulate_buy_key_response.total_needed_from_user > user_paid_amount {
        return Err(ContractError::InsufficientFundsToPayDuringBuy {
            needed: simulate_buy_key_response.total_needed_from_user,
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
                amount: simulate_buy_key_response.key_issuer_fee,
            }],
        }),
        // TODO: P0: distribute key holder fee to all key holders
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

    USERS.update(
        deps.storage,
        key_issuer_addr_ref,
        |key_issuer| match key_issuer {
            None => Err(ContractError::UserNotExist {}),
            Some(key_issuer) => {
                if key_issuer.issued_key.clone().unwrap().supply <= data.amount {
                    return Err(ContractError::CannotSellLastKey {
                        sell: data.amount,
                        total_supply: key_issuer.issued_key.unwrap().supply,
                    });
                }
                let updated_key_issuer = User {
                    addr: key_issuer.addr,
                    social_media_handle: key_issuer.social_media_handle,
                    issued_key: Some(Key {
                        supply: key_issuer.issued_key.clone().unwrap().supply - data.amount,
                        key_trading_fee_config: key_issuer
                            .issued_key
                            .clone()
                            .unwrap()
                            .key_trading_fee_config,
                        qa_fee_config: key_issuer.issued_key.unwrap().qa_fee_config,
                    }),
                };
                Ok(updated_key_issuer)
            }
        },
    )?;

    let user_previous_hold_amount = ALL_USERS_HOLDINGS
        .may_load(deps.storage, (sender_addr_ref, key_issuer_addr_ref))?
        .unwrap_or_default();
    if user_previous_hold_amount < data.amount {
        return Err(ContractError::InsufficientKeysToSell {
            sell: data.amount,
            available: user_previous_hold_amount,
        });
    }

    let simulate_sell_key_response: SimulateSellKeyResponse = deps.querier.query_wasm_smart(
        env.contract.address,
        &QueryMsg::QuerySimulateSellKey(QuerySimulateSellKeyMsg {
            key_issuer_addr: data.key_issuer_addr.clone(),
            amount: data.amount,
        }),
    )?;

    if simulate_sell_key_response.total_needed_from_user > user_paid_amount {
        return Err(ContractError::InsufficientFundsToPayDuringSell {
            needed: simulate_sell_key_response.total_needed_from_user,
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
        // TODO: P0: distribute key holder fee to all key holders
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
