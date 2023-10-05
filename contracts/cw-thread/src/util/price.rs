use cosmwasm_std::{Addr, Deps, Uint128, Uint64};
use thread::config::FeeShareConfig;

use crate::state::{CONFIG, USERS};

pub fn calculate_price(supply: Uint128, amount: Uint128) -> Uint128 {
    let supply_minus_one: Uint128 = supply - Uint128::one();
    let two_times_supply_minus_one_plus_pne =
        supply_minus_one * Uint128::from(2_u8) + Uint128::one();

    let sum1 = if supply.is_zero() {
        Uint128::zero()
    } else {
        supply_minus_one * supply * two_times_supply_minus_one_plus_pne / Uint128::from(6_u8)
    };

    let supply_minus_one_plus_amount = supply_minus_one + amount;
    let supply_plus_amount = supply + amount;
    let two_times_supply_minus_one_plus_amount_plus_one =
        supply_minus_one_plus_amount * Uint128::from(2_u8) + Uint128::one();

    let sum2 = if supply.is_zero() && amount == Uint128::one() {
        Uint128::zero()
    } else {
        supply_minus_one_plus_amount
            * supply_plus_amount
            * two_times_supply_minus_one_plus_amount_plus_one
            / Uint128::from(6_u8)
    };

    let summation = sum2 - sum1;
    // 1_000_000 because 1 LUNA = 1_000_000 uluna
    summation * Uint128::from(1_000_000_u64) / Uint128::from(16_000_u32)
}

pub fn multiply_percentage(price: Uint128, percentage: Uint64) -> Uint128 {
    price * Uint128::from(percentage) / Uint128::from(100_u8)
}

pub fn lookup_trading_fee_percentage_of_key(deps: Deps, user_addr: &Addr) -> Uint64 {
    let user = USERS.load(deps.storage, user_addr).unwrap();
    user.trading_fee_percentage_of_key.unwrap_or(
        CONFIG
            .load(deps.storage)
            .unwrap()
            .default_trading_fee_percentage_of_key,
    )
}

pub fn lookup_ask_fee_percentage_of_key(deps: Deps, user_addr: &Addr) -> Uint64 {
    let user = USERS.load(deps.storage, user_addr).unwrap();
    user.ask_fee_percentage_of_key.unwrap_or(
        CONFIG
            .load(deps.storage)
            .unwrap()
            .default_ask_fee_percentage_of_key,
    )
}

pub fn lookup_reply_fee_percentage_of_key(deps: Deps, user_addr: &Addr) -> Uint64 {
    let user = USERS.load(deps.storage, user_addr).unwrap();
    user.reply_fee_percentage_of_key.unwrap_or(
        CONFIG
            .load(deps.storage)
            .unwrap()
            .default_reply_fee_percentage_of_key,
    )
}

pub fn lookup_key_trading_fee_share_config(deps: Deps, user_addr: &Addr) -> FeeShareConfig {
    let user = USERS.load(deps.storage, user_addr).unwrap();
    user.key_trading_fee_share_config.unwrap_or(
        CONFIG
            .load(deps.storage)
            .unwrap()
            .default_key_trading_fee_share_config,
    )
}

pub fn lookup_thread_fee_share_config(deps: Deps, user_addr: &Addr) -> FeeShareConfig {
    let user = USERS.load(deps.storage, user_addr).unwrap();
    user.thread_fee_share_config.unwrap_or(
        CONFIG
            .load(deps.storage)
            .unwrap()
            .default_thread_fee_share_config,
    )
}

pub fn lookup_ask_fee_to_thread_creator_percentage_of_key(deps: Deps, user_addr: &Addr) -> Uint64 {
    lookup_key_trading_fee_share_config(deps, user_addr).key_issuer_fee_percentage
}