use cosmwasm_std::Uint128;

use thread::{
    key_holder::KeyHolder,
    msg::{QueryMsg, QueryUserMsg, UserResponse},
    user::User,
    user_holding::UserHolding,
};

pub mod helpers;
use crate::helpers::{
    assert_key_holders, assert_key_supply, assert_user_holdings,
    link_social_media_and_register_key, proper_instantiate, register_user, SOCIAL_MEDIA_HANDLE_1,
};

#[test]
fn test_registration_admin_can_register_key_on_behalf_of_user() {
    let (mut app, cw_thread_contract_addr, _, registration_admin_addr, _, user_1_addr, _) =
        proper_instantiate();
    register_user(&mut app, &cw_thread_contract_addr, &user_1_addr);
    link_social_media_and_register_key(
        &mut app,
        &cw_thread_contract_addr,
        &registration_admin_addr,
        &user_1_addr,
        SOCIAL_MEDIA_HANDLE_1,
    );

    let query_user_1_res: UserResponse = app
        .wrap()
        .query_wasm_smart(
            cw_thread_contract_addr.clone(),
            &QueryMsg::QueryUser(QueryUserMsg {
                user_addr: user_1_addr.to_string(),
            }),
        )
        .unwrap();
    assert_eq!(
        query_user_1_res,
        UserResponse {
            user: User {
                addr: user_1_addr.clone(),
                social_media_handle: Some(SOCIAL_MEDIA_HANDLE_1.to_string()),
                issued_key: true,
                trading_fee_percentage_of_key: None,
                ask_fee_percentage_of_key: None,
                reply_fee_percentage_of_key: None,
                key_trading_fee_share_config: None,
                thread_fee_share_config: None
            }
        }
    );

    assert_key_supply(&app, &cw_thread_contract_addr, &user_1_addr, Uint128::one());
    assert_user_holdings(
        &app,
        &cw_thread_contract_addr,
        &user_1_addr,
        vec![UserHolding {
            issuer_addr: user_1_addr.clone(),
            amount: Uint128::one(),
        }],
    );
    assert_key_holders(
        &app,
        &cw_thread_contract_addr,
        &user_1_addr,
        vec![KeyHolder {
            holder_addr: user_1_addr.clone(),
            amount: Uint128::one(),
        }],
    );
}
