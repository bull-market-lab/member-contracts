use cosmwasm_std::Uint128;

pub fn calculate_price(supply: Uint128, amount: Uint128) -> Uint128 {
    let supply_minus_one = supply.checked_sub(Uint128::from(1 as u8)).unwrap();
    let two_times_supply_minus_one_plus_pne = supply_minus_one
        .checked_mul(Uint128::from(2 as u8))
        .unwrap()
        .checked_add(Uint128::from(1 as u8))
        .unwrap();

    let sum1 = if supply.is_zero() {
        Uint128::zero()
    } else {
        supply_minus_one
            .checked_mul(supply)
            .unwrap()
            .checked_mul(two_times_supply_minus_one_plus_pne)
            .unwrap()
            .checked_div(Uint128::from(6 as u8))
            .unwrap()
    };

    let supply_minus_one_plus_amount = supply_minus_one.checked_add(amount).unwrap();
    let supply_plus_amount = supply.checked_add(amount).unwrap();
    let two_times_supply_minus_one_plus_amount_plus_one = supply_minus_one_plus_amount
        .checked_mul(Uint128::from(2 as u8))
        .unwrap()
        .checked_add(Uint128::from(1 as u8))
        .unwrap();

    let sum2 = if supply.is_zero() && amount == Uint128::from(1 as u8) {
        Uint128::zero()
    } else {
        supply_minus_one_plus_amount
            .checked_mul(supply_plus_amount)
            .unwrap()
            .checked_mul(two_times_supply_minus_one_plus_amount_plus_one)
            .unwrap()
            .checked_div(Uint128::from(6 as u8))
            .unwrap()
    };

    let summation = sum2.checked_sub(sum1).unwrap();
    summation
        // 1_000_000 because 1 LUNA = 1_000_000 uluna
        .checked_mul(Uint128::from(1_000_000 as u64))
        .unwrap()
        .checked_div(Uint128::from(16_000 as u32))
        .unwrap()
}

pub fn calculate_fee(price: Uint128, fee_percentage: Uint128) -> Uint128 {
    price
        .checked_mul(fee_percentage)
        .unwrap()
        .checked_div(Uint128::from(100 as u8))
        .unwrap()
}
