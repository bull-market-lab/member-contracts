use thread::{
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
                issued_key: false,
                trading_fee_percentage_of_key: None,
                ask_fee_percentage_of_key: None,
                ask_fee_to_thread_creator_percentage_of_key: None,
                reply_fee_percentage_of_key: None,
                key_trading_fee_share_config: None,
                thread_fee_share_config: None
            }
        }
    );
}
