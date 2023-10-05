use cosmwasm_std::{Coin, Uint128};
use cw_multi_test::Executor;

use thread::{
    key_holder::KeyHolder,
    msg::{
        BuyKeyMsg, CostToBuyKeyResponse, CostToSellKeyResponse, ExecuteMsg, QueryCostToBuyKeyMsg,
        QueryCostToSellKeyMsg, QueryMsg, SellKeyMsg,
    },
    user_holding::UserHolding,
};

pub mod helpers;
use crate::helpers::{
    assert_balance, assert_key_holders, assert_key_supply, assert_user_holdings,
    get_fund_from_faucet, link_social_media_and_register_key, proper_instantiate, register_user,
    FEE_DENOM, SOCIAL_MEDIA_HANDLE_1,
};

#[test]
fn cw_thread_contract_multi_test_2_users_buy_and_sell_keys() {
    let (
        mut app,
        cw_thread_contract_addr,
        _,
        registration_admin_addr,
        fee_collector_addr,
        user_1_addr,
        user_2_addr,
    ) = proper_instantiate();

    let default_supply = Uint128::one();
    let uint_128_amount_30 = Uint128::from(30_u8);
    let uint_128_amount_25 = Uint128::from(25_u8);
    let uint_128_amount_15 = Uint128::from(15_u8);
    let uint_128_amount_10 = Uint128::from(10_u8);
    let uint_128_amount_20 = Uint128::from(20_u8);

    register_user(&mut app, &cw_thread_contract_addr, &user_1_addr);
    link_social_media_and_register_key(
        &mut app,
        &cw_thread_contract_addr,
        &registration_admin_addr,
        &user_1_addr,
        SOCIAL_MEDIA_HANDLE_1,
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
    get_fund_from_faucet(
        &mut app,
        user_1_addr.clone(),
        query_user_1_simulate_buy_key_res.total_needed_from_user,
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

    // User 2 buys 25 amount of user 1's keys
    let query_user_2_simulate_buy_key_res: CostToBuyKeyResponse = app
        .wrap()
        .query_wasm_smart(
            cw_thread_contract_addr.clone(),
            &QueryMsg::QueryCostToBuyKey(QueryCostToBuyKeyMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_25,
            }),
        )
        .unwrap();
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
            amount: uint_128_amount_25,
        }),
        &[Coin {
            denom: FEE_DENOM.to_string(),
            amount: query_user_2_simulate_buy_key_res.total_needed_from_user,
        }],
    )
    .unwrap();

    // User 2 sells 15 amount of user 1's keys
    let query_user_2_simulate_sell_key_res: CostToSellKeyResponse = app
        .wrap()
        .query_wasm_smart(
            cw_thread_contract_addr.clone(),
            &QueryMsg::QueryCostToSellKey(QueryCostToSellKeyMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_15,
            }),
        )
        .unwrap();
    get_fund_from_faucet(
        &mut app,
        user_2_addr.clone(),
        query_user_2_simulate_sell_key_res.total_needed_from_user,
    );
    app.execute_contract(
        user_2_addr.clone(),
        cw_thread_contract_addr.clone(),
        &ExecuteMsg::SellKey(SellKeyMsg {
            key_issuer_addr: user_1_addr.to_string(),
            amount: uint_128_amount_15,
        }),
        &[Coin {
            denom: FEE_DENOM.to_string(),
            amount: query_user_2_simulate_sell_key_res.total_needed_from_user,
        }],
    )
    .unwrap();

    // User 1 sells 10 amount of its own keys
    let query_user_1_simulate_sell_key_res: CostToSellKeyResponse = app
        .wrap()
        .query_wasm_smart(
            cw_thread_contract_addr.clone(),
            &QueryMsg::QueryCostToSellKey(QueryCostToSellKeyMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_10,
            }),
        )
        .unwrap();
    get_fund_from_faucet(
        &mut app,
        user_1_addr.clone(),
        query_user_1_simulate_sell_key_res.total_needed_from_user,
    );
    app.execute_contract(
        user_1_addr.clone(),
        cw_thread_contract_addr.clone(),
        &ExecuteMsg::SellKey(SellKeyMsg {
            key_issuer_addr: user_1_addr.to_string(),
            amount: uint_128_amount_10,
        }),
        &[Coin {
            denom: FEE_DENOM.to_string(),
            amount: query_user_1_simulate_sell_key_res.total_needed_from_user,
        }],
    )
    .unwrap();

    // ========== Check balances, key supply, user holdings, and key holders ==========

    assert_key_supply(
        &app,
        &cw_thread_contract_addr,
        &user_1_addr,
        default_supply + uint_128_amount_30 + uint_128_amount_25
            - uint_128_amount_15
            - uint_128_amount_10,
    );

    assert_user_holdings(
        &app,
        &cw_thread_contract_addr,
        &user_1_addr,
        vec![UserHolding {
            issuer_addr: user_1_addr.clone(),
            amount: default_supply + uint_128_amount_30 - uint_128_amount_10,
        }],
    );
    assert_user_holdings(
        &app,
        &cw_thread_contract_addr,
        &user_2_addr,
        vec![UserHolding {
            issuer_addr: user_1_addr.clone(),
            amount: uint_128_amount_25 - uint_128_amount_15,
        }],
    );
    assert_key_holders(
        &app,
        &cw_thread_contract_addr,
        &user_1_addr,
        vec![
            KeyHolder {
                holder_addr: user_1_addr.clone(),
                amount: default_supply + uint_128_amount_20,
            },
            KeyHolder {
                holder_addr: user_2_addr.clone(),
                amount: uint_128_amount_10,
            },
        ],
    );

    assert_balance(
        &app,
        &fee_collector_addr,
        query_user_1_simulate_buy_key_res.protocol_fee
            + query_user_2_simulate_buy_key_res.protocol_fee
            + query_user_2_simulate_sell_key_res.protocol_fee
            + query_user_1_simulate_sell_key_res.protocol_fee,
        FEE_DENOM,
    );

    assert_balance(
        &app,
        &cw_thread_contract_addr,
        query_user_1_simulate_buy_key_res.price + query_user_2_simulate_buy_key_res.price
            - query_user_2_simulate_sell_key_res.price
            - query_user_1_simulate_sell_key_res.price
            // TODO: why do we nee to add 2? divide has rounding error?
            + Uint128::from(2_u8),
        FEE_DENOM,
    );

    assert_balance(
        &app,
        &user_1_addr,
        query_user_1_simulate_buy_key_res.key_issuer_fee
            + query_user_2_simulate_buy_key_res.key_issuer_fee
            + query_user_2_simulate_sell_key_res.key_issuer_fee
            + query_user_1_simulate_sell_key_res.key_issuer_fee
            + query_user_1_simulate_sell_key_res.price
            + query_user_1_simulate_buy_key_res.key_holder_fee
            + query_user_2_simulate_buy_key_res.key_holder_fee
            + query_user_2_simulate_sell_key_res.key_holder_fee * Uint128::from(31_u8)
                / Uint128::from(41_u8)
            + query_user_1_simulate_sell_key_res.key_holder_fee * Uint128::from(21_u8)
                / Uint128::from(31_u8),
        FEE_DENOM,
    );

    assert_balance(
        &app,
        &user_2_addr,
        query_user_2_simulate_sell_key_res.price
            + query_user_2_simulate_sell_key_res.key_holder_fee * Uint128::from(10_u8)
                / Uint128::from(41_u8)
            + query_user_1_simulate_sell_key_res.key_holder_fee * Uint128::from(10_u8)
                / Uint128::from(31_u8),
        FEE_DENOM,
    );
}
