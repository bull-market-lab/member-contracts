use cosmwasm_std::{Deps, StdResult, Uint128};

use thread::msg::{QuerySimulateAskMsg, SimulateAskResponse};

use crate::{
    state::USERS,
    util::price::{calculate_fee, calculate_price},
};

pub fn query_simulate_ask(deps: Deps, data: QuerySimulateAskMsg) -> StdResult<SimulateAskResponse> {
    let key = USERS
        .load(deps.storage, &data.ask_to_addr)?
        .issued_key
        .unwrap();

    let price_for_single_key = calculate_price(key.supply, Uint128::one());
    // TODO: store multiply per character to config
    // TODO: P0: revise the formula
    let price = price_for_single_key * key.qa_fee_config.ask_fee_in_key_price_percentage
        / Uint128::from(100 as u128)
        * data.content_len
        / Uint128::from(50 as u128);
    let key_issuer_fee = calculate_fee(price, key.qa_fee_config.key_issuer_fee_percentage);
    let key_holder_fee = calculate_fee(price, key.qa_fee_config.key_holder_fee_percentage);
    let protocol_fee = calculate_fee(price, key.qa_fee_config.protocol_fee_percentage);
    let total_needed_from_user = price + protocol_fee + key_issuer_fee + key_holder_fee;

    Ok(SimulateAskResponse {
        protocol_fee,
        key_issuer_fee,
        key_holder_fee,
        total_needed_from_user,
    })
}
