use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};

use friend::msg::{BuyKeyMsg, QuerySimulateBuyKeyMsg, SellKeyMsg, SimulateBuyKeyResponse};

use crate::ContractError;

pub fn buy_key(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    data: BuyKeyMsg,
) -> Result<Response, ContractError> {
    let simulate_buy_key_response: SimulateBuyKeyResponse = deps.querier.query_wasm_smart(
        env.contract.address,
        &QuerySimulateBuyKeyMsg {
            key_issuer_addr: data.key_issuer_addr.clone(),
            amount: data.amount,
        },
    )?;

    let price_and_fee = simulate_buy_key_response.price
        + simulate_buy_key_response.protocol_fee
        + simulate_buy_key_response.key_issuer_fee;

    if price_and_fee > info.funds[0].amount {
        return Err(ContractError::InsufficientFunds {
            needed: price_and_fee,
            available: info.funds[0].amount,
        });
    }

    // let mut config = CONFIG.load(deps.storage)?;
    // if info.sender != config.admin {
    //     return Err(ContractError::Unauthorized {});
    // }

    // config.admin = match data.admin {
    //     None => config.admin,
    //     Some(data) => deps.api.addr_validate(data.as_str())?,
    // };

    // config.key_register_admin = match data.key_register_admin {
    //     None => config.key_register_admin,
    //     Some(data) => deps.api.addr_validate(data.as_str())?,
    // };

    // config.fee_collector = match data.fee_collector {
    //     None => config.fee_collector,
    //     Some(data) => deps.api.addr_validate(data.as_str())?,
    // };

    // config.protocol_fee_percentage = data
    //     .protocol_fee_percentage
    //     .unwrap_or(config.protocol_fee_percentage);
    // config.key_issuer_fee_percentage = data
    //     .key_issuer_fee_percentage
    //     .unwrap_or(config.key_issuer_fee_percentage);

    // if config.protocol_fee_percentage.u64() > 100 {
    //     return Err(ContractError::ProtocolFeeTooHigh {});
    // }

    // if config.key_issuer_fee_percentage.u64() > 100 {
    //     return Err(ContractError::KeyIssuerFeeTooHigh {});
    // }

    // CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

pub fn sell_key(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    data: SellKeyMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}
