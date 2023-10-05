use cosmwasm_std::{Coin, Uint128};
use cw_multi_test::Executor;

use thread::{
    key_holder::KeyHolder,
    msg::{BuyKeyMsg, CostToBuyKeyResponse, ExecuteMsg, QueryCostToBuyKeyMsg, QueryMsg},
    user_holding::UserHolding,
};

use crate::helpers::{
    assert_balance, assert_key_holders, assert_key_supply, assert_user_holdings,
    get_fund_from_faucet, link_social_media_and_register_key, print_balance, proper_instantiate,
    register_user, FEE_DENOM, SOCIAL_MEDIA_HANDLE_1,
};

#[test]
fn test_buy_key_happy_case() {
    let (
        mut app,
        cw_thread_contract_addr,
        admin_addr,
        registration_admin_addr,
        fee_collector_addr,
        user_1_addr,
        user_2_addr,
    ) = proper_instantiate();

    let default_supply: Uint128 = Uint128::one();
    let uint_128_amount_30: Uint128 = Uint128::from(30_u8);
    let uint_128_amount_20: Uint128 = Uint128::from(20_u8);

    register_user(&mut app, &cw_thread_contract_addr, &user_1_addr);
    link_social_media_and_register_key(
        &mut app,
        &cw_thread_contract_addr,
        &registration_admin_addr,
        &user_1_addr,
        SOCIAL_MEDIA_HANDLE_1,
    );

    assert_key_supply(&app, &cw_thread_contract_addr, &user_1_addr, Uint128::one());

    print_balance(
        &app,
        &cw_thread_contract_addr,
        &admin_addr,
        &fee_collector_addr,
        &registration_admin_addr,
        &user_1_addr,
        &user_2_addr,
    );

    // User 1 buy 30 amount of its own keys
    let query_user_1_simulate_buy_key_res: CostToBuyKeyResponse = app
        .wrap()
        .query_wasm_smart(
            cw_thread_contract_addr.clone(),
            &QueryMsg::QueryCostToBuyKey(QueryCostToBuyKeyMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_30,
            }),
        )
        .unwrap();

    get_fund_from_faucet(&mut app, user_1_addr.clone(), Uint128::one());

    get_fund_from_faucet(
        &mut app,
        user_1_addr.clone(),
        query_user_1_simulate_buy_key_res.total_needed_from_user - Uint128::one(),
    );
    app.execute_contract(
        user_1_addr.clone(),
        cw_thread_contract_addr.clone(),
        &ExecuteMsg::BuyKey(BuyKeyMsg {
            key_issuer_addr: user_1_addr.to_string(),
            amount: uint_128_amount_30,
        }),
        &[Coin {
            denom: FEE_DENOM.to_string(),
            amount: query_user_1_simulate_buy_key_res.total_needed_from_user,
        }],
    )
    .unwrap();
    print_balance(
        &app,
        &cw_thread_contract_addr,
        &admin_addr,
        &fee_collector_addr,
        &registration_admin_addr,
        &user_1_addr,
        &user_2_addr,
    );
    assert_balance(
        &app,
        &cw_thread_contract_addr,
        query_user_1_simulate_buy_key_res.price,
        FEE_DENOM,
    );
    assert_balance(
        &app,
        &user_1_addr,
        query_user_1_simulate_buy_key_res.key_issuer_fee
            + query_user_1_simulate_buy_key_res.key_holder_fee,
        FEE_DENOM,
    );
    assert_balance(
        &app,
        &fee_collector_addr,
        query_user_1_simulate_buy_key_res.protocol_fee,
        FEE_DENOM,
    );
    assert_key_supply(
        &app,
        &cw_thread_contract_addr,
        &user_1_addr,
        default_supply + uint_128_amount_30,
    );
    assert_user_holdings(
        &app,
        &cw_thread_contract_addr,
        &user_1_addr,
        vec![UserHolding {
            issuer_addr: user_1_addr.clone(),
            amount: default_supply + uint_128_amount_30,
        }],
    );
    assert_key_holders(
        &app,
        &cw_thread_contract_addr,
        &user_1_addr,
        vec![KeyHolder {
            holder_addr: user_1_addr.clone(),
            amount: default_supply + uint_128_amount_30,
        }],
    );

    // User 2 buy 20 amount of user 1's keys
    let query_user_2_simulate_buy_key_res: CostToBuyKeyResponse = app
        .wrap()
        .query_wasm_smart(
            cw_thread_contract_addr.clone(),
            &QueryMsg::QueryCostToBuyKey(QueryCostToBuyKeyMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_20,
            }),
        )
        .unwrap();
    assert_eq!(
        query_user_2_simulate_buy_key_res,
        CostToBuyKeyResponse {
            price: Uint128::from(2_091_875_u32),
            protocol_fee: Uint128::from(10_459_u32),
            key_issuer_fee: Uint128::from(52_296_u32),
            key_holder_fee: Uint128::from(52_296_u32),
            total_needed_from_user: Uint128::from(2_206_926_u32),
        }
    );
    get_fund_from_faucet(
        &mut app,
        user_2_addr.clone(),
        query_user_2_simulate_buy_key_res.total_needed_from_user,
    );
    app.execute_contract(
        user_2_addr.clone(),
        cw_thread_contract_addr.clone(),
        &ExecuteMsg::BuyKey(BuyKeyMsg {
            key_issuer_addr: user_1_addr.to_string(),
            amount: uint_128_amount_20,
        }),
        &[Coin {
            denom: FEE_DENOM.to_string(),
            amount: query_user_2_simulate_buy_key_res.total_needed_from_user,
        }],
    )
    .unwrap();

    // ========== Check balances, key supply, user holdings, and key holders ==========

    assert_balance(
        &app,
        &cw_thread_contract_addr,
        query_user_2_simulate_buy_key_res.price + query_user_1_simulate_buy_key_res.price,
        FEE_DENOM,
    );
    assert_balance(
        &app,
        &user_1_addr,
        query_user_2_simulate_buy_key_res.key_issuer_fee
            + query_user_2_simulate_buy_key_res.key_holder_fee
            + query_user_1_simulate_buy_key_res.key_issuer_fee
            + query_user_1_simulate_buy_key_res.key_holder_fee,
        FEE_DENOM,
    );
    assert_balance(&app, &user_2_addr, Uint128::zero(), FEE_DENOM);
    assert_balance(
        &app,
        &fee_collector_addr,
        query_user_2_simulate_buy_key_res.protocol_fee
            + query_user_1_simulate_buy_key_res.protocol_fee,
        FEE_DENOM,
    );
    assert_key_supply(
        &app,
        &cw_thread_contract_addr,
        &user_1_addr,
        default_supply + uint_128_amount_30 + uint_128_amount_20,
    );
    assert_user_holdings(
        &app,
        &cw_thread_contract_addr,
        &user_1_addr,
        vec![UserHolding {
            issuer_addr: user_1_addr.clone(),
            amount: default_supply + uint_128_amount_30,
        }],
    );
    assert_user_holdings(
        &app,
        &cw_thread_contract_addr,
        &user_2_addr,
        vec![UserHolding {
            issuer_addr: user_1_addr.clone(),
            amount: uint_128_amount_20,
        }],
    );
    assert_key_holders(
        &app,
        &cw_thread_contract_addr,
        &user_1_addr,
        vec![
            KeyHolder {
                holder_addr: user_1_addr.clone(),
                amount: default_supply + uint_128_amount_30,
            },
            KeyHolder {
                holder_addr: user_2_addr.clone(),
                amount: uint_128_amount_20,
            },
        ],
    );
}
