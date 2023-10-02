use cosmwasm_std::{Deps, StdResult, Uint128};

use thread::msg::{
    CostToBuyKeyResponse, CostToSellKeyResponse, KeySupplyResponse, QueryCostToBuyKeyMsg,
    QueryCostToSellKeyMsg, QueryKeySupplyMsg,
};

use crate::{
    state::{CONFIG, KEY_SUPPLY},
    util::price::{
        calculate_price, lookup_key_trading_fee_share_config, lookup_trading_fee_percentage_of_key,
        multiply_percentage,
    },
};

pub fn query_key_supply(deps: Deps, data: QueryKeySupplyMsg) -> StdResult<KeySupplyResponse> {
    let key_issuer_addr_ref = &deps
        .api
        .addr_validate(data.key_issuer_addr.as_str())
        .unwrap();

    let supply = KEY_SUPPLY.load(deps.storage, key_issuer_addr_ref).unwrap();

    Ok(KeySupplyResponse { supply })
}

fn shared(
    deps: Deps,
    key_issuer_addr: String,
    amount: Uint128,
) -> (Uint128, Uint128, Uint128, Uint128) {
    let key_issuer_addr_ref = &deps.api.addr_validate(key_issuer_addr.as_str()).unwrap();

    let supply = KEY_SUPPLY.load(deps.storage, key_issuer_addr_ref).unwrap();

    let price = calculate_price(supply, amount);
    let fee = multiply_percentage(
        price,
        lookup_trading_fee_percentage_of_key(deps, key_issuer_addr_ref),
    );

    let key_trading_fee_share_config =
        lookup_key_trading_fee_share_config(deps, key_issuer_addr_ref);
    let key_issuer_fee =
        multiply_percentage(fee, key_trading_fee_share_config.key_issuer_fee_percentage);
    let key_holder_fee =
        multiply_percentage(fee, key_trading_fee_share_config.key_holder_fee_percentage);

    let protocol_fee_percentage = CONFIG
        .load(deps.storage)
        .unwrap()
        .protocol_fee_config
        .key_trading_fee_percentage;
    let protocol_fee = multiply_percentage(fee, protocol_fee_percentage);

    (price, key_issuer_fee, key_holder_fee, protocol_fee)
}

pub fn query_cost_to_buy_key(
    deps: Deps,
    data: QueryCostToBuyKeyMsg,
) -> StdResult<CostToBuyKeyResponse> {
    let (price, key_issuer_fee, key_holder_fee, protocol_fee) =
        shared(deps, data.key_issuer_addr, data.amount);

    let total_needed_from_user = price + protocol_fee + key_issuer_fee + key_holder_fee;

    Ok(CostToBuyKeyResponse {
        price,
        protocol_fee,
        key_issuer_fee,
        key_holder_fee,
        total_needed_from_user,
    })
}

pub fn query_cost_to_sell_key(
    deps: Deps,
    data: QueryCostToSellKeyMsg,
) -> StdResult<CostToSellKeyResponse> {
    let (price, key_issuer_fee, key_holder_fee, protocol_fee) =
        shared(deps, data.key_issuer_addr, data.amount);

    let total_needed_from_user = protocol_fee + key_issuer_fee + key_holder_fee;

    Ok(CostToSellKeyResponse {
        price,
        protocol_fee,
        key_issuer_fee,
        key_holder_fee,
        total_needed_from_user,
    })
}
