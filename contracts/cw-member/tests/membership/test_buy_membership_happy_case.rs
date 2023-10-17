use cosmwasm_std::{Coin, Uint128, Uint64};
use cw_multi_test::Executor;

use member::{
    msg::{
        BuyMembershipMsg, CostToBuyMembershipResponse, ExecuteMsg, QueryCostToBuyMembershipMsg,
        QueryMsg,
    },
    user::{Member, Membership},
};

use crate::helpers::{
    assert_balance, assert_member_count, assert_members, assert_membership_supply,
    assert_memberships, enable_membership, get_fund_from_faucet, link_social_media, print_balance,
    proper_instantiate, register_user, FEE_DENOM, SOCIAL_MEDIA_HANDLE_1,
};

#[test]
fn test_buy_membership_happy_case() {
    let (
        mut app,
        cw_member_contract_addr,
        admin_addr,
        registration_admin_addr,
        fee_collector_addr,
        user_1_addr,
        user_2_addr,
    ) = proper_instantiate();

    let default_supply: Uint128 = Uint128::one();
    let uint_128_amount_30: Uint128 = Uint128::from(30_u8);
    let uint_128_amount_20: Uint128 = Uint128::from(20_u8);

    register_user(&mut app, &cw_member_contract_addr, &user_1_addr).unwrap();
    register_user(&mut app, &cw_member_contract_addr, &user_2_addr).unwrap();

    let user_1_id = Uint64::one();
    let user_2_id = Uint64::from(2_u8);

    link_social_media(
        &mut app,
        &cw_member_contract_addr,
        &registration_admin_addr,
        user_1_id,
        SOCIAL_MEDIA_HANDLE_1,
    )
    .unwrap();
    enable_membership(
        &mut app,
        &cw_member_contract_addr,
        &registration_admin_addr,
        user_1_id,
    )
    .unwrap();

    assert_membership_supply(&app, &cw_member_contract_addr, user_1_id, Uint128::one());

    assert_member_count(&app, &cw_member_contract_addr, user_1_id, Uint128::one());

    print_balance(
        &app,
        &cw_member_contract_addr,
        &admin_addr,
        &fee_collector_addr,
        &registration_admin_addr,
        &user_1_addr,
        &user_2_addr,
    );

    // ================ User 1 buy 30 amount of its own memberships ================
    let query_user_1_simulate_buy_membership_res: CostToBuyMembershipResponse = app
        .wrap()
        .query_wasm_smart(
            cw_member_contract_addr.clone(),
            &QueryMsg::QueryCostToBuyMembership(QueryCostToBuyMembershipMsg {
                membership_issuer_user_id: user_1_id,
                amount: uint_128_amount_30,
            }),
        )
        .unwrap();

    get_fund_from_faucet(&mut app, user_1_addr.clone(), Uint128::one());

    get_fund_from_faucet(
        &mut app,
        user_1_addr.clone(),
        query_user_1_simulate_buy_membership_res.total_needed_from_user - Uint128::one(),
    );
    app.execute_contract(
        user_1_addr.clone(),
        cw_member_contract_addr.clone(),
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

    // ========== Check balances, membership supply, user holdings, and membership holders ==========

    print_balance(
        &app,
        &cw_member_contract_addr,
        &admin_addr,
        &fee_collector_addr,
        &registration_admin_addr,
        &user_1_addr,
        &user_2_addr,
    );
    assert_balance(
        &app,
        &cw_member_contract_addr,
        query_user_1_simulate_buy_membership_res.price,
        FEE_DENOM,
    );
    assert_balance(
        &app,
        &user_1_addr,
        query_user_1_simulate_buy_membership_res.issuer_fee
            + query_user_1_simulate_buy_membership_res.all_members_fee,
        FEE_DENOM,
    );
    assert_balance(
        &app,
        &fee_collector_addr,
        query_user_1_simulate_buy_membership_res.protocol_fee,
        FEE_DENOM,
    );

    assert_membership_supply(
        &app,
        &cw_member_contract_addr,
        user_1_id,
        default_supply + uint_128_amount_30,
    );

    assert_member_count(&app, &cw_member_contract_addr, user_1_id, Uint128::one());

    assert_memberships(
        &app,
        &cw_member_contract_addr,
        user_1_id,
        vec![Membership {
            issuer_user_id: user_1_id,
            amount: default_supply + uint_128_amount_30,
        }],
        1,
        1,
    );
    assert_members(
        &app,
        &cw_member_contract_addr,
        user_1_id,
        vec![Member {
            member_user_id: user_1_id,
            amount: default_supply + uint_128_amount_30,
        }],
        1,
        1,
    );

    // ================ User 2 buy 20 amount of user 1's memberships ================

    let query_user_2_simulate_buy_membership_res: CostToBuyMembershipResponse = app
        .wrap()
        .query_wasm_smart(
            cw_member_contract_addr.clone(),
            &QueryMsg::QueryCostToBuyMembership(QueryCostToBuyMembershipMsg {
                membership_issuer_user_id: user_1_id,
                amount: uint_128_amount_20,
            }),
        )
        .unwrap();
    assert_eq!(
        query_user_2_simulate_buy_membership_res,
        CostToBuyMembershipResponse {
            price: Uint128::from(2_091_875_u32),
            protocol_fee: Uint128::from(10_459_u32),
            issuer_fee: Uint128::from(83_674_u32),
            all_members_fee: Uint128::from(20_918_u32),
            total_needed_from_user: Uint128::from(2_206_926_u32),
        }
    );
    get_fund_from_faucet(
        &mut app,
        user_2_addr.clone(),
        query_user_2_simulate_buy_membership_res.total_needed_from_user,
    );
    app.execute_contract(
        user_2_addr.clone(),
        cw_member_contract_addr.clone(),
        &ExecuteMsg::BuyMembership(BuyMembershipMsg {
            membership_issuer_user_id: user_1_id,
            amount: uint_128_amount_20,
        }),
        &[Coin {
            denom: FEE_DENOM.to_string(),
            amount: query_user_2_simulate_buy_membership_res.total_needed_from_user,
        }],
    )
    .unwrap();

    // ========== Check balances, membership supply, user holdings, and membership holders ==========

    assert_balance(
        &app,
        &cw_member_contract_addr,
        query_user_2_simulate_buy_membership_res.price
            + query_user_1_simulate_buy_membership_res.price,
        FEE_DENOM,
    );
    assert_balance(
        &app,
        &user_1_addr,
        query_user_2_simulate_buy_membership_res.issuer_fee
            + query_user_2_simulate_buy_membership_res.all_members_fee
            + query_user_1_simulate_buy_membership_res.issuer_fee
            + query_user_1_simulate_buy_membership_res.all_members_fee,
        FEE_DENOM,
    );
    assert_balance(&app, &user_2_addr, Uint128::zero(), FEE_DENOM);
    assert_balance(
        &app,
        &fee_collector_addr,
        query_user_2_simulate_buy_membership_res.protocol_fee
            + query_user_1_simulate_buy_membership_res.protocol_fee,
        FEE_DENOM,
    );
    assert_membership_supply(
        &app,
        &cw_member_contract_addr,
        user_1_id,
        default_supply + uint_128_amount_30 + uint_128_amount_20,
    );
    assert_memberships(
        &app,
        &cw_member_contract_addr,
        user_1_id,
        vec![Membership {
            issuer_user_id: user_1_id,
            amount: default_supply + uint_128_amount_30,
        }],
        1,
        1,
    );
    assert_memberships(
        &app,
        &cw_member_contract_addr,
        user_2_id,
        vec![Membership {
            issuer_user_id: user_1_id,
            amount: uint_128_amount_20,
        }],
        1,
        1,
    );
    assert_members(
        &app,
        &cw_member_contract_addr,
        user_1_id,
        vec![
            Member {
                member_user_id: user_1_id,
                amount: default_supply + uint_128_amount_30,
            },
            Member {
                member_user_id: user_2_id,
                amount: uint_128_amount_20,
            },
        ],
        2,
        2,
    );
}
