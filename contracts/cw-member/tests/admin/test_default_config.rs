use cosmwasm_std::{Addr, Uint64};

use member::{
    config::Config,
    msg::{ConfigResponse, QueryConfigMsg, QueryMsg},
};

use crate::helpers::{proper_instantiate, ADMIN, FEE_COLLECTOR, REGISTRATION_ADMIN};

#[test]
fn test_default_config() {
    let (app, cw_member_contract_addr, _, _, _, _, _) = proper_instantiate();
    let config_res: ConfigResponse = app
        .wrap()
        .query_wasm_smart(
            cw_member_contract_addr.clone(),
            &QueryMsg::QueryConfig(QueryConfigMsg {}),
        )
        .unwrap();
    assert_eq!(
        config_res,
        ConfigResponse {
            config: Config {
                admin_addr: Addr::unchecked(ADMIN),
                distribution_contract_addr: None,
                enabled: false,
                enable_open_registration: false,
                registration_admin_addr: Addr::unchecked(REGISTRATION_ADMIN),
                protocol_fee_collector_addr: Addr::unchecked(FEE_COLLECTOR),
                fee_denom: "uluna".to_string(),
                protocol_fee_membership_trading_fee_percentage: Uint64::from(10_u64),
                default_trading_fee_percentage_of_membership: Uint64::from(5_u64),
                default_share_to_issuer_percentage: Uint64::from(80_u64),
                default_share_to_all_members_percentage: Uint64::from(20_u64),
            }
        }
    );
}
