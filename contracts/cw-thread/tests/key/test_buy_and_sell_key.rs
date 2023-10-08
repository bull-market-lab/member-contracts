use cosmwasm_std::{Coin, Uint128};
use cw_multi_test::Executor;

use thread::{
    key_holder::MembershipHolder,
    msg::{
        BuyMembershipMsg, CostToBuyMembershipResponse, CostToSellMembershipResponse, ExecuteMsg,
        QueryCostToBuyMembershipMsg, QueryCostToSellMembershipMsg, QueryMsg, SellMembershipMsg,
    },
    user_holding::Membership,
};

use crate::helpers::{
    assert_balance, assert_key_holders, assert_key_supply, assert_memberships,
    get_fund_from_faucet, link_social_media_and_enable_membership, print_balance,
    proper_instantiate, register_user, FEE_DENOM, SOCIAL_MEDIA_HANDLE_1,
};

#[test]
fn test_buy_and_sell_membership() {
    let (
        mut app,
        cw_thread_contract_addr,
        admin_addr,
        registration_admin_addr,
        fee_collector_addr,
        user_1_addr,
        user_2_addr,
    ) = proper_instantiate();

    let default_supply = Uint128::one();
    let uint_128_amount_30 = Uint128::from(30_u8);

    register_user(&mut app, &cw_thread_contract_addr, &user_1_addr);
    link_social_media_and_enable_membership(
        &mut app,
        &cw_thread_contract_addr,
        &registration_admin_addr,
        &user_1_addr,
        SOCIAL_MEDIA_HANDLE_1,
    );

    // User 1 buy 30 amount of its own keys
    let query_user_1_simulate_buy_membership_res: CostToBuyMembershipResponse = app
        .wrap()
        .query_wasm_smart(
            cw_thread_contract_addr.clone(),
            &QueryMsg::QueryCostToBuyMembership(QueryCostToBuyMembershipMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_30,
            }),
        )
        .unwrap();

    get_fund_from_faucet(
        &mut app,
        user_1_addr.clone(),
        query_user_1_simulate_buy_membership_res.total_needed_from_user,
    );

    app.execute_contract(
        user_1_addr.clone(),
        cw_thread_contract_addr.clone(),
        &ExecuteMsg::BuyMembership(BuyMembershipMsg {
            key_issuer_addr: user_1_addr.to_string(),
            amount: uint_128_amount_30,
        }),
        &[Coin {
            denom: FEE_DENOM.to_string(),
            amount: query_user_1_simulate_buy_membership_res.total_needed_from_user,
        }],
    )
    .unwrap();

    // User 1 tries to sell 30 amount of its own keys and succeeds
    let query_user_1_simulate_sell_membership_res: CostToSellMembershipResponse = app
        .wrap()
        .query_wasm_smart(
            cw_thread_contract_addr.clone(),
            &QueryMsg::QueryCostToSellMembership(QueryCostToSellMembershipMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_30,
            }),
        )
        .unwrap();

    // Price should be the same as buying 30 keys because user 1 is the only user buying / selling so far
    get_fund_from_faucet(
        &mut app,
        user_1_addr.clone(),
        query_user_1_simulate_sell_membership_res.total_needed_from_user,
    );

    print_balance(
        &app,
        &cw_thread_contract_addr,
        &admin_addr,
        &fee_collector_addr,
        &registration_admin_addr,
        &user_1_addr,
        &user_2_addr,
    );

    app.execute_contract(
        user_1_addr.clone(),
        cw_thread_contract_addr.clone(),
        &ExecuteMsg::SellMembership(SellMembershipMsg {
            key_issuer_addr: user_1_addr.to_string(),
            amount: uint_128_amount_30,
        }),
        &[Coin {
            denom: FEE_DENOM.to_string(),
            amount: query_user_1_simulate_sell_membership_res.total_needed_from_user,
        }],
    )
    .unwrap();

    // ========== Check balances, key supply, user holdings, and key holders ==========

    assert_balance(&app, &cw_thread_contract_addr, Uint128::zero(), FEE_DENOM);
    assert_balance(
        &app,
        &user_1_addr,
        query_user_1_simulate_sell_membership_res.key_holder_fee
            + query_user_1_simulate_sell_membership_res.key_issuer_fee
            + query_user_1_simulate_buy_membership_res.key_issuer_fee
            + query_user_1_simulate_buy_membership_res.key_holder_fee
            + query_user_1_simulate_buy_membership_res.price,
        FEE_DENOM,
    );
    assert_balance(
        &app,
        &fee_collector_addr,
        query_user_1_simulate_sell_membership_res.protocol_fee
            + query_user_1_simulate_buy_membership_res.protocol_fee,
        FEE_DENOM,
    );
    assert_key_supply(&app, &cw_thread_contract_addr, &user_1_addr, default_supply);
    assert_memberships(
        &app,
        &cw_thread_contract_addr,
        &user_1_addr,
        vec![Membership {
            issuer_addr: user_1_addr.clone(),
            amount: default_supply,
        }],
    );
    assert_key_holders(
        &app,
        &cw_thread_contract_addr,
        &user_1_addr,
        vec![MembershipHolder {
            holder_addr: user_1_addr.clone(),
            amount: default_supply,
        }],
    );
}
