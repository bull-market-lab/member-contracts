use cosmwasm_std::{Coin, Uint128, Uint64};
use cw_multi_test::Executor;

use membership::{
    msg::{
        BuyMembershipMsg, CostToBuyMembershipResponse, CostToSellMembershipResponse, ExecuteMsg,
        QueryCostToBuyMembershipMsg, QueryCostToSellMembershipMsg, QueryMsg, SellMembershipMsg,
    },
    user::{Member, Membership},
};

use crate::helpers::{
    assert_balance, assert_member_count, assert_members, assert_membership_supply,
    assert_memberships, get_fund_from_faucet, link_social_media_and_enable_membership,
    print_balance, proper_instantiate, register_user, FEE_DENOM, SOCIAL_MEDIA_HANDLE_1,
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
    let user_1_id = Uint64::one();

    link_social_media_and_enable_membership(
        &mut app,
        &cw_thread_contract_addr,
        &registration_admin_addr,
        user_1_id,
        SOCIAL_MEDIA_HANDLE_1,
    );

    // User 1 buy 30 amount of its own memberships
    let query_user_1_simulate_buy_membership_res: CostToBuyMembershipResponse = app
        .wrap()
        .query_wasm_smart(
            cw_thread_contract_addr.clone(),
            &QueryMsg::QueryCostToBuyMembership(QueryCostToBuyMembershipMsg {
                membership_issuer_user_id: user_1_id,
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
            membership_issuer_user_id: user_1_id,
            amount: uint_128_amount_30,
        }),
        &[Coin {
            denom: FEE_DENOM.to_string(),
            amount: query_user_1_simulate_buy_membership_res.total_needed_from_user,
        }],
    )
    .unwrap();

    // User 1 tries to sell 30 amount of its own memberships and succeeds
    let query_user_1_simulate_sell_membership_res: CostToSellMembershipResponse = app
        .wrap()
        .query_wasm_smart(
            cw_thread_contract_addr.clone(),
            &QueryMsg::QueryCostToSellMembership(QueryCostToSellMembershipMsg {
                membership_issuer_user_id: user_1_id,
                amount: uint_128_amount_30,
            }),
        )
        .unwrap();

    // Price should be the same as buying 30 memberships because user 1 is the only user buying / selling so far
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
            membership_issuer_user_id: user_1_id,
            amount: uint_128_amount_30,
        }),
        &[Coin {
            denom: FEE_DENOM.to_string(),
            amount: query_user_1_simulate_sell_membership_res.total_needed_from_user,
        }],
    )
    .unwrap();

    // ========== Check balances, membership supply, user holdings, and membership holders ==========

    assert_balance(&app, &cw_thread_contract_addr, Uint128::zero(), FEE_DENOM);
    assert_balance(
        &app,
        &user_1_addr,
        query_user_1_simulate_sell_membership_res.all_members_fee
            + query_user_1_simulate_sell_membership_res.issuer_fee
            + query_user_1_simulate_buy_membership_res.issuer_fee
            + query_user_1_simulate_buy_membership_res.all_members_fee
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

    assert_membership_supply(&app, &cw_thread_contract_addr, user_1_id, default_supply);

    assert_member_count(&app, &cw_thread_contract_addr, user_1_id, Uint128::one());

    assert_memberships(
        &app,
        &cw_thread_contract_addr,
        user_1_id,
        vec![Membership {
            issuer_user_id: user_1_id,
            amount: default_supply,
        }],
        1,
        1,
    );
    assert_members(
        &app,
        &cw_thread_contract_addr,
        user_1_id,
        vec![Member {
            member_user_id: user_1_id,
            amount: default_supply,
        }],
        1,
        1,
    );
}
