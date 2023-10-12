use cosmwasm_std::{Uint128, Uint64};

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
