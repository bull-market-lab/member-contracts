use cosmwasm_std::{Coin, Uint128, Uint64};
use cw_multi_test::Executor;

use cw_member::ContractError;
use member::msg::{BuyMembershipMsg, ExecuteMsg};

use crate::helpers::{
    assert_err, assert_member_count, assert_membership_supply, get_fund_from_faucet,
    link_social_media_and_enable_membership, print_balance, proper_instantiate, register_user,
    FEE_DENOM, SOCIAL_MEDIA_HANDLE_1,
};

#[test]
fn test_buy_membership_insufficient_funds() {
    let (
        mut app,
        cw_thread_contract_addr,
        admin_addr,
        registration_admin_addr,
        fee_collector_addr,
        user_1_addr,
        user_2_addr,
    ) = proper_instantiate();

    let uint_128_amount_30: Uint128 = Uint128::from(30_u8);

    register_user(&mut app, &cw_thread_contract_addr, &user_1_addr);
    let user_1_id = Uint64::one();

    link_social_media_and_enable_membership(
        &mut app,
        &cw_thread_contract_addr,
        &registration_admin_addr,
        user_1_id,
        SOCIAL_MEDIA_HANDLE_1,
    );

    assert_membership_supply(&app, &cw_thread_contract_addr, user_1_id, Uint128::one());

    assert_member_count(&app, &cw_thread_contract_addr, user_1_id, Uint128::one());

    print_balance(
        &app,
        &cw_thread_contract_addr,
        &admin_addr,
        &fee_collector_addr,
        &registration_admin_addr,
        &user_1_addr,
        &user_2_addr,
    );

    // User 1 buy 30 amount of its own memberships but fails because it does not have enough funds

    get_fund_from_faucet(&mut app, user_1_addr.clone(), Uint128::one());

    assert_err(
        app.execute_contract(
            user_1_addr.clone(),
            cw_thread_contract_addr.clone(),
            &ExecuteMsg::BuyMembership(BuyMembershipMsg {
                membership_issuer_user_id: user_1_id,
                amount: uint_128_amount_30,
            }),
            &[Coin {
                denom: FEE_DENOM.to_string(),
                amount: Uint128::one(),
            }],
        ),
        ContractError::InsufficientFundsToPayDuringBuy {
            needed: Uint128::from(623_436_u32),
            available: Uint128::one(),
        },
    );
}
