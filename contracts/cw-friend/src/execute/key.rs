use cosmwasm_std::{BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128};

use friend::{
    config::Config,
    msg::{
        BuyKeyMsg, QuerySimulateBuyKeyMsg, QuerySimulateSellKeyMsg, SellKeyMsg,
        SimulateBuyKeyResponse, SimulateSellKeyResponse,
    },
};

use crate::{state::ALL_USERS_HOLDINGS, ContractError};

pub fn buy_key(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    data: BuyKeyMsg,
    config: Config,
    user_paid_amount: Uint128,
) -> Result<Response, ContractError> {
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

    ALL_USERS_HOLDINGS.update(deps.storage, info.sender.clone(), |existing_holdings| {
        match existing_holdings {
            None => {
                let mut new_holdings = Vec::new();
                new_holdings.push(data.key_issuer_addr.clone());
                Ok(new_holdings)
            }
            Some(mut existing_holdings) => {
                existing_holdings.push(data.key_issuer_addr.clone());
                Ok(existing_holdings)
            }
        }
    })?;

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
