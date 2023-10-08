use membership::{
    msg::{QueryMsg, QueryUserMsg, UserResponse},
    user::User,
};

use crate::helpers::{proper_instantiate, register_user};

#[test]
fn test_user_can_register_itself() {
    let (mut app, cw_thread_contract_addr, _, _, _, user_1_addr, _) = proper_instantiate();
    register_user(&mut app, &cw_thread_contract_addr, &user_1_addr);
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
                social_media_handle: None,
                membership_enabled: false,
                trading_fee_percentage_of_membership: None,
                share_to_issuer_percentage: None,
                share_to_all_members_percentage: None
            }
        }
    );
}
