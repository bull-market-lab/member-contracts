use cosmwasm_std::Uint64;

use member::{
    config::{Config, FeeConfig, ProtocolFeeConfig},
    msg::{ConfigResponse, QueryConfigMsg, QueryMsg},
};
use shared::fee_share_config::FeeShareConfig;

use crate::helpers::{proper_instantiate, update_config};

#[test]
fn test_only_admin_can_update_config() {
    let (mut app, cw_member_contract_addr, admin_addr, _, _, user_1_addr, user_2_addr) =
        proper_instantiate();

    update_config(
        &mut app,
        &cw_member_contract_addr,
        &admin_addr,
        Some(user_1_addr.to_string()),
        Some(user_2_addr.to_string()),
        Some(user_2_addr.to_string()),
        Some(user_2_addr.to_string()),
        Some(Uint64::from(20_u64)),
        Some(Uint64::from(80_u64)),
        Some(Uint64::from(70_u64)),
        Some(Uint64::from(30_u64)),
    )
    .unwrap();

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
                admin_addr: user_1_addr,
                distribution_contract_addr: Some(user_2_addr.clone()),
                enabled: false,
                enable_open_registration: false,
                registration_admin_addr: user_2_addr.clone(),
                protocol_fee_collector_addr: user_2_addr.clone(),
                default_fee_config: FeeConfig {
                    fee_denom: "uluna".to_string(),
                    trading_fee_percentage_of_membership: Uint64::from(80_u64),
                },
                protocol_fee_config: ProtocolFeeConfig {
                    membership_trading_fee_percentage: Uint64::from(20_u64),
                },
                default_fee_share_config: FeeShareConfig {
                    share_to_issuer_percentage: Uint64::from(70_u64),
                    share_to_all_members_percentage: Uint64::from(30_u64),
                }
            }
        }
    );
}
