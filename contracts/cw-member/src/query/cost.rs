use cosmwasm_std::{Deps, StdResult, Uint128};

use member::{
    config::Config,
    msg::{
        CostToBuyMembershipResponse, CostToSellMembershipResponse, QueryCostToBuyMembershipMsg,
        QueryCostToSellMembershipMsg,
    },
};

use crate::{
    state::{ALL_USERS, CONFIG},
    util::price::{
        calculate_price, lookup_fee_share_to_all_members_percentage,
        lookup_fee_share_to_issuer_percentage, lookup_trading_fee_percentage_of_membership,
        multiply_percentage,
    },
};

fn shared(
    deps: Deps,
    config: Config,
    membership_issuer_user_id: u64,
    supply: Uint128,
    amount: Uint128,
) -> (Uint128, Uint128, Uint128, Uint128) {
    let price = calculate_price(supply, amount);

    let issuer = ALL_USERS()
        .idx
        .id
        .item(deps.storage, membership_issuer_user_id)
        .unwrap()
        .unwrap()
        .1;

    let fee = multiply_percentage(
        price,
        lookup_trading_fee_percentage_of_membership(
            config.default_trading_fee_percentage_of_membership,
            issuer.config.trading_fee_percentage_of_membership,
        ),
    );

    let issuer_fee = multiply_percentage(
        fee,
        lookup_fee_share_to_issuer_percentage(
            config.default_share_to_issuer_percentage,
            issuer.config.share_to_issuer_percentage,
        ),
    );

    let all_members_fee = multiply_percentage(
        fee,
        lookup_fee_share_to_all_members_percentage(
            config.default_share_to_all_members_percentage,
            issuer.config.share_to_all_members_percentage,
        ),
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
    config: Config,
) -> StdResult<CostToBuyMembershipResponse> {
    let membership_issuer_user_id = data.membership_issuer_user_id.u64();

    let old_supply = ALL_USERS()
        .idx
        .id
        .item(deps.storage, membership_issuer_user_id)?
        .unwrap()
        .1
        .membership_issued_by_me
        .unwrap()
        .membership_supply;

    let (price, issuer_fee, all_members_fee, protocol_fee) = shared(
        deps,
        config,
        membership_issuer_user_id,
        old_supply,
        data.amount,
    );

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
    config: Config,
) -> StdResult<CostToSellMembershipResponse> {
    let membership_issuer_user_id = data.membership_issuer_user_id.u64();

    let old_supply = ALL_USERS()
        .idx
        .id
        .item(deps.storage, membership_issuer_user_id)?
        .unwrap()
        .1
        .membership_issued_by_me
        .unwrap()
        .membership_supply;

    let (price, issuer_fee, all_members_fee, protocol_fee) = shared(
        deps,
        config,
        membership_issuer_user_id,
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
