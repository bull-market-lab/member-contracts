use cosmwasm_std::{Deps, StdResult};

use friend::msg::{
    QueryKeySupplyMsg, QuerySimulateBuyKeyMsg, QuerySimulateSellKeyMsg, SimulateBuyKeyResponse,
    SimulateSellKeyResponse,
};

use crate::{
    state::{CONFIG, USERS},
    util::price::{calculate_fee, calculate_price},
};

pub fn query_key_supply(
    deps: Deps,
    data: QueryKeySupplyMsg,
) -> StdResult<friend::msg::KeySupplyResponse> {
    let user = USERS.load(deps.storage, &data.key_issuer_addr)?;
    let supply = user.issued_key.unwrap().supply;
    Ok(friend::msg::KeySupplyResponse { supply })
}

pub fn query_simulate_buy_key(
    deps: Deps,
    data: QuerySimulateBuyKeyMsg,
) -> StdResult<SimulateBuyKeyResponse> {
    let config = CONFIG.load(deps.storage)?;
    let supply = USERS
        .load(deps.storage, &data.key_issuer_addr)?
        .issued_key
        .unwrap()
        .supply;

    let price = calculate_price(supply, data.amount);
    let key_issuer_fee = calculate_fee(price, config.key_issuer_fee_percentage);
    let protocol_fee = calculate_fee(price, config.protocol_fee_percentage);
    let total_needed_from_user = price + protocol_fee + key_issuer_fee;

    Ok(SimulateBuyKeyResponse {
        price,
        protocol_fee,
        key_issuer_fee,
        total_needed_from_user,
    })
}

pub fn query_simulate_sell_key(
    deps: Deps,
    data: QuerySimulateSellKeyMsg,
) -> StdResult<SimulateSellKeyResponse> {
    let config = CONFIG.load(deps.storage)?;
    let supply = USERS
        .load(deps.storage, &data.key_issuer_addr)?
        .issued_key
        .unwrap()
        .supply;

    let price = calculate_price(supply - data.amount, data.amount);
    let key_issuer_fee = calculate_fee(price, config.key_issuer_fee_percentage);
    let protocol_fee = calculate_fee(price, config.protocol_fee_percentage);
    let total_needed_from_user = protocol_fee + key_issuer_fee;

    Ok(SimulateSellKeyResponse {
        price,
        protocol_fee,
        key_issuer_fee,
        total_needed_from_user,
    })
}
