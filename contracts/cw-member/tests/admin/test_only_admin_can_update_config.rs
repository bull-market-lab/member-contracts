use cosmwasm_std::{Addr, Uint64};

use cw_member::ContractError;
use member::{
    config::Config,
    msg::{ConfigResponse, QueryConfigMsg, QueryMsg},
};

use crate::helpers::{
    assert_err, proper_instantiate, update_config, ADMIN, FEE_COLLECTOR, REGISTRATION_ADMIN,
};

#[test]
fn test_only_admin_can_update_config() {
    let (mut app, cw_member_contract_addr, admin_addr, _, _, user_1_addr, _) = proper_instantiate();

    assert_err(
        update_config(
            &mut app,
            &cw_member_contract_addr,
            &user_1_addr,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        ContractError::OnlyRegistrationAdminCanLinkSocialMediaOnBehalfOfUser {},
    );

    // let config_res: ConfigResponse = app
    //     .wrap()
    //     .query_wasm_smart(
    //         cw_member_contract_addr.clone(),
    //         &QueryMsg::QueryConfig(QueryConfigMsg {}),
    //     )
    //     .unwrap();
    // assert_eq!(
    //     config_res,
    //     ConfigResponse {
    //         config: Config {
    //             admin_addr: Addr::unchecked(ADMIN),
    //             distribution_contract_addr: None,
    //             enabled: false,
    //             enable_open_registration: false,
    //             registration_admin_addr: Addr::unchecked(REGISTRATION_ADMIN),
    //             protocol_fee_collector_addr: Addr::unchecked(FEE_COLLECTOR),
    //             fee_denom: "uluna".to_string(),
    //             protocol_fee_membership_trading_fee_percentage: Uint64::from(10_u64),
    //             default_trading_fee_percentage_of_membership: Uint64::from(5_u64),
    //             default_share_to_issuer_percentage: Uint64::from(80_u64),
    //             default_share_to_all_members_percentage: Uint64::from(20_u64),
    //         }
    //     }
    // );
}
