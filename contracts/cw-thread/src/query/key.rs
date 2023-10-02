use cosmwasm_std::{Deps, StdResult};

use thread::msg::{
    QuerySimulateBuyKeyMsg, QuerySimulateSellKeyMsg, SimulateBuyKeyResponse,
    SimulateSellKeyResponse,
};

use crate::{
    state::USERS,
    util::price::{calculate_fee, calculate_price},
};

pub fn query_simulate_buy_key(
    deps: Deps,
    data: QuerySimulateBuyKeyMsg,
) -> StdResult<SimulateBuyKeyResponse> {
    let key = USERS
        .load(deps.storage, &data.key_issuer_addr)?
        .issued_key
        .unwrap();

    let price = calculate_price(key.supply, data.amount);
    let key_issuer_fee = calculate_fee(price, key.key_trading_fee_config.key_issuer_fee_percentage);
    let key_holder_fee = calculate_fee(price, key.key_trading_fee_config.key_holder_fee_percentage);
    let protocol_fee = calculate_fee(price, key.key_trading_fee_config.protocol_fee_percentage);
    let total_needed_from_user = price + protocol_fee + key_issuer_fee + key_holder_fee;

    Ok(SimulateBuyKeyResponse {
        price,
        protocol_fee,
        key_issuer_fee,
        key_holder_fee,
        total_needed_from_user,
    })
}

pub fn query_simulate_sell_key(
    deps: Deps,
    data: QuerySimulateSellKeyMsg,
) -> StdResult<SimulateSellKeyResponse> {
    let key = USERS
        .load(deps.storage, &data.key_issuer_addr)?
        .issued_key
        .unwrap();

    let price = calculate_price(key.supply - data.amount, data.amount);
    let key_issuer_fee = calculate_fee(price, key.key_trading_fee_config.key_issuer_fee_percentage);
    let key_holder_fee = calculate_fee(price, key.key_trading_fee_config.key_holder_fee_percentage);
    let protocol_fee = calculate_fee(price, key.key_trading_fee_config.protocol_fee_percentage);
    let total_needed_from_user = protocol_fee + key_issuer_fee + key_holder_fee;

    Ok(SimulateSellKeyResponse {
        price,
        protocol_fee,
        key_issuer_fee,
        key_holder_fee,
        total_needed_from_user,
    })
}
