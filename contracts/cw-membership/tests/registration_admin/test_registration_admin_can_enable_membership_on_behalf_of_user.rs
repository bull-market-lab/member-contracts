use cosmwasm_std::Uint128;

use membership::{
    member::Member,
    membership::Membership,
    msg::{QueryMsg, QueryUserMsg, UserResponse},
    user::User,
};

use crate::helpers::{
    assert_members, assert_membership_supply, assert_memberships,
    link_social_media_and_enable_membership, proper_instantiate, register_user,
    SOCIAL_MEDIA_HANDLE_1, assert_member_count,
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
                membership_enabled: true,
                trading_fee_percentage_of_membership: None,
                share_to_issuer_percentage: None,
                share_to_all_members_percentage: None
            }
        }
    );

    assert_membership_supply(&app, &cw_thread_contract_addr, &user_1_addr, Uint128::one());

    assert_member_count(&app, &cw_thread_contract_addr, &user_1_addr, Uint128::one());

    assert_memberships(
        &app,
        &cw_thread_contract_addr,
        &user_1_addr,
        vec![Membership {
            issuer_addr: user_1_addr.clone(),
            amount: Uint128::one(),
        }],
    );

    assert_members(
        &app,
        &cw_thread_contract_addr,
        &user_1_addr,
        vec![Member {
            holder_addr: user_1_addr.clone(),
            amount: Uint128::one(),
        }],
    );
}
