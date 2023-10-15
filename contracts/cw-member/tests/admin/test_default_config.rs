use cosmwasm_std::Uint64;

use member::{
    config::{Config, FeeConfig, ProtocolFeeConfig},
    msg::{ConfigResponse, QueryConfigMsg, QueryMsg},
};
use shared::fee_share_config::FeeShareConfig;

use crate::helpers::proper_instantiate;

#[test]
fn test_default_config() {
    let (
        app,
        cw_member_contract_addr,
        admin_addr,
        registration_admin_addr,
        protocol_fee_collector_addr,
        _,
        _,
    ) = proper_instantiate();
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
                admin_addr,
                distribution_contract_addr: None,
                enabled: false,
                enable_open_registration: false,
                registration_admin_addr,
                protocol_fee_collector_addr,
                default_fee_config: FeeConfig {
                    fee_denom: "uluna".to_string(),
                    trading_fee_percentage_of_membership: Uint64::from(5_u64),
                },
                protocol_fee_config: ProtocolFeeConfig {
                    membership_trading_fee_percentage: Uint64::from(10_u64),
                },
                default_fee_share_config: FeeShareConfig {
                    share_to_issuer_percentage: Uint64::from(80_u64),
                    share_to_all_members_percentage: Uint64::from(20_u64),
                }
            }
        }
    );
}
