use cosmwasm_std::{Addr, Deps, StdResult, Uint128};

use membership::msg::{
    CostToBuyMembershipResponse, CostToSellMembershipResponse, QueryCostToBuyMembershipMsg,
    QueryCostToSellMembershipMsg,
};

use crate::{
    state::{CONFIG, MEMBERSHIP_SUPPLY},
    util::price::{
        calculate_price, lookup_trading_fee_percentage_of_membership,
        lookup_trading_fee_share_to_all_members_percentage,
        lookup_trading_fee_share_to_issuer_percentage, multiply_percentage,
    },
};

fn shared(
    deps: Deps,
    membership_issuer_addr_ref: &Addr,
    supply: Uint128,
    amount: Uint128,
) -> (Uint128, Uint128, Uint128, Uint128) {
    let price = calculate_price(supply, amount);
    let fee = multiply_percentage(
        price,
        lookup_trading_fee_percentage_of_membership(deps, membership_issuer_addr_ref),
    );

    // let membership_trading_fee_share_config =
    //     lookup_membership_trading_fee_share_config(deps, membership_issuer_addr_ref);
    let issuer_fee = multiply_percentage(
        fee,
        lookup_trading_fee_share_to_issuer_percentage(deps, membership_issuer_addr_ref),
    );
    let all_members_fee = multiply_percentage(
        fee,
        lookup_trading_fee_share_to_all_members_percentage(deps, membership_issuer_addr_ref),
    );

    let protocol_fee = multiply_percentage(
        fee,
        CONFIG
            .load(deps.storage)
            .unwrap()
            .protocol_fee_membership_trading_fee_percentage,
    );

    (price, issuer_fee, all_members_fee, protocol_fee)
}

pub fn query_cost_to_buy_membership(
    deps: Deps,
    data: QueryCostToBuyMembershipMsg,
) -> StdResult<CostToBuyMembershipResponse> {
    let membership_issuer_addr_ref = &deps
        .api
        .addr_validate(data.membership_issuer_addr.as_str())
        .unwrap();

    let old_supply = MEMBERSHIP_SUPPLY
        .load(deps.storage, membership_issuer_addr_ref)
        .unwrap();

    let (price, issuer_fee, all_members_fee, protocol_fee) =
        shared(deps, membership_issuer_addr_ref, old_supply, data.amount);

    let total_needed_from_user = price + protocol_fee + issuer_fee + all_members_fee;

    Ok(CostToBuyMembershipResponse {
        price,
        protocol_fee,
        issuer_fee,
        all_members_fee,
        total_needed_from_user,
    })
}

pub fn query_cost_to_sell_membership(
    deps: Deps,
    data: QueryCostToSellMembershipMsg,
) -> StdResult<CostToSellMembershipResponse> {
    let membership_issuer_addr_ref = &deps
        .api
        .addr_validate(data.membership_issuer_addr.as_str())
        .unwrap();

    let old_supply: Uint128 = MEMBERSHIP_SUPPLY
        .load(deps.storage, membership_issuer_addr_ref)
        .unwrap();

    let (price, issuer_fee, all_members_fee, protocol_fee) = shared(
        deps,
        membership_issuer_addr_ref,
        // We need this to make sure price is the same across buy and sell
        // e.g. old supply is 5, now buy 10 memberships, new supply is 15
        // Now sell 10 memberships, new supply is 5, price to buy 10 memberships should be the same as price to sell 10 memberships
        // Because before supply and after supply is the same
        old_supply - data.amount,
        data.amount,
    );

    let total_needed_from_user = protocol_fee + issuer_fee + all_members_fee;

    Ok(CostToSellMembershipResponse {
        price,
        protocol_fee,
        issuer_fee,
        all_members_fee,
        total_needed_from_user,
    })
}
