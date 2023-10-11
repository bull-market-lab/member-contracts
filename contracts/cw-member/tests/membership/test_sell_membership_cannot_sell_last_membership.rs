use cosmwasm_std::{Coin, Uint128, Uint64};
use cw_multi_test::Executor;

use cw_member::ContractError;
use member::msg::{ExecuteMsg, SellMembershipMsg};

use crate::helpers::{
    assert_err, get_fund_from_faucet, link_social_media_and_enable_membership, print_balance,
    proper_instantiate, register_user, FEE_DENOM, SOCIAL_MEDIA_HANDLE_1,
};

#[test]
fn test_sell_membership_cannot_sell_last_key() {
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

    register_user(&mut app, &cw_thread_contract_addr, &user_1_addr);
    let user_1_id = Uint64::one();

    link_social_media_and_enable_membership(
        &mut app,
        &cw_thread_contract_addr,
        &registration_admin_addr,
        user_1_id,
        SOCIAL_MEDIA_HANDLE_1,
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

    // User 1 tries to sell 1 amount of its own keys but fails because key supply cannot go to 0
    get_fund_from_faucet(&mut app, user_1_addr.clone(), Uint128::one());
    assert_err(
        app.execute_contract(
            user_1_addr.clone(),
            cw_thread_contract_addr.clone(),
            &ExecuteMsg::SellMembership(SellMembershipMsg {
                membership_issuer_user_id: user_1_id,
                amount: default_supply,
            }),
            &[Coin {
                denom: FEE_DENOM.to_string(),
                amount: Uint128::one(),
            }],
        ),
        ContractError::CannotSellLastMembership {
            sell: default_supply,
            total_supply: default_supply,
        },
    );
}
