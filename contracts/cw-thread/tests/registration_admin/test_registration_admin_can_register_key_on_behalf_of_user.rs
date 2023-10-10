use cosmwasm_std::Uint128;

use thread::{
    key_holder::MembershipHolder,
    msg::{QueryMsg, QueryUserByAddrMsg, UserResponse},
    user::User,
    user_holding::Membership,
};

use crate::helpers::{
    assert_key_holders, assert_key_supply, assert_memberships,
    link_social_media_and_enable_membership, proper_instantiate, register_user,
    SOCIAL_MEDIA_HANDLE_1,
};

#[test]
fn test_registration_admin_can_enable_membership_on_behalf_of_user() {
    let (mut app, cw_thread_contract_addr, _, registration_admin_addr, _, user_1_addr, _) =
        proper_instantiate();
    register_user(&mut app, &cw_thread_contract_addr, &user_1_addr);
    link_social_media_and_enable_membership(
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
            &QueryMsg::QueryUserByAddr(QueryUserByAddrMsg {
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
                ask_fee_to_thread_creator_percentage_of_key: None,
                reply_fee_percentage_of_key: None,
                key_trading_fee_share_config: None,
                thread_fee_share_config: None
            }
        }
    );

    assert_key_supply(&app, &cw_thread_contract_addr, &user_1_addr, Uint128::one());
    assert_memberships(
        &app,
        &cw_thread_contract_addr,
        &user_1_addr,
        vec![Membership {
            issuer_addr: user_1_addr.clone(),
            amount: Uint128::one(),
        }],
    );
    assert_key_holders(
        &app,
        &cw_thread_contract_addr,
        &user_1_addr,
        vec![MembershipHolder {
            holder_addr: user_1_addr.clone(),
            amount: Uint128::one(),
        }],
    );
}
