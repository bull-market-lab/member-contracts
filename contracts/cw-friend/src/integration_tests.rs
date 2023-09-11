#[cfg(test)]
mod tests {
    use anyhow::Result as AnyResult;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, AppResponse, Contract, ContractWrapper, Executor};

    use friend::{
        key::Key,
        key_holder::KeyHolder,
        msg::{
            ExecuteMsg, InstantiateMsg, KeyHoldersResponse, KeySupplyResponse, QueryKeyHoldersMsg,
            QueryKeySupplyMsg, QueryMsg, QueryUserHoldingsMsg, QueryUserMsg,
            RegisterSocialMediaAndKeyMsg, UserHoldingsResponse, UserResponse,
        },
        user::User,
        user_holding::UserHolding,
    };

    use crate::{
        contract::{execute, instantiate, query},
        ContractError,
    };

    const ADMIN: &str = "terra1";
    const KEY_REGISTER_ADMIN: &str = "terra2";
    const FEE_COLLECTOR: &str = "terra3";

    const USER_1: &str = "terra4";
    const USER_2: &str = "terra5";

    const SOCIAL_MEDIA_HANDLE_1: &str = "twitter1";
    const SOCIAL_MEDIA_HANDLE_2: &str = "twitter2";

    const FEE_DENOM: &str = "uluna";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(ADMIN),
                    vec![Coin {
                        denom: FEE_DENOM.to_string(),
                        amount: Uint128::new(1),
                    }],
                )
                .unwrap();
        })
    }

    fn contract_cw_friend() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    fn proper_instantiate() -> (App, Addr, Addr, Addr, Addr, Addr, Addr) {
        let mut app = mock_app();
        let cw_friend_contract_code_id = app.store_code(contract_cw_friend());

        let msg = InstantiateMsg {
            admin_addr: Some(ADMIN.to_string()),
            key_register_admin_addr: Some(KEY_REGISTER_ADMIN.to_string()),
            protocol_fee_collector_addr: Some(FEE_COLLECTOR.to_string()),
            fee_denom: Some(FEE_DENOM.to_string()),
            protocol_fee_percentage: Uint128::from(5 as u8),
            key_issuer_fee_percentage: Uint128::from(5 as u8),
        };
        let cw_friend_addr = app
            .instantiate_contract(
                cw_friend_contract_code_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "cw_friend",
                None,
            )
            .unwrap();

        let admin_addr = Addr::unchecked(ADMIN.to_string());
        let key_register_admin_addr = Addr::unchecked(KEY_REGISTER_ADMIN.to_string());
        let fee_collector_addr = Addr::unchecked(FEE_COLLECTOR.to_string());
        let user_1_addr = Addr::unchecked(USER_1.to_string());
        let user_2_addr = Addr::unchecked(USER_2.to_string());

        (
            app,
            cw_friend_addr,
            admin_addr,
            key_register_admin_addr,
            fee_collector_addr,
            user_1_addr,
            user_2_addr,
        )
    }

    fn assert_err(res: AnyResult<AppResponse>, err: ContractError) {
        match res {
            Ok(_) => panic!("Result was not an error"),
            Err(generic_err) => {
                let contract_err: ContractError = generic_err.downcast().unwrap();
                assert_eq!(contract_err, err);
            }
        }
    }

    #[test]
    fn cw_friend_contract_multi_test_user_can_register_itself() {
        let (mut app, cw_friend_contract_addr, _, _, _, user_1_addr, _) = proper_instantiate();

        app.execute_contract(
            user_1_addr.clone(),
            cw_friend_contract_addr.clone(),
            &ExecuteMsg::Register(),
            &[],
        )
        .unwrap();
        let query_user_1_res: UserResponse = app
            .wrap()
            .query_wasm_smart(
                cw_friend_contract_addr.clone(),
                &QueryMsg::QueryUser(QueryUserMsg {
                    user_addr: user_1_addr.clone(),
                }),
            )
            .unwrap();
        assert_eq!(
            query_user_1_res,
            UserResponse {
                user: User {
                    addr: user_1_addr.clone(),
                    social_media_handle: None,
                    issued_key: None
                }
            }
        );
    }

    #[test]
    fn cw_friend_contract_multi_test_user_cannot_register_key_by_itself() {
        let (mut app, cw_friend_contract_addr, _, _, _, user_1_addr, _) = proper_instantiate();

        app.execute_contract(
            user_1_addr.clone(),
            cw_friend_contract_addr.clone(),
            &ExecuteMsg::Register(),
            &[],
        )
        .unwrap();

        assert_err(
            app.execute_contract(
                user_1_addr.clone(),
                cw_friend_contract_addr.clone(),
                &ExecuteMsg::RegisterSocialMediaAndKey(RegisterSocialMediaAndKeyMsg {
                    user_addr: user_1_addr.clone(),
                    social_media_handle: SOCIAL_MEDIA_HANDLE_1.to_string(),
                }),
                &[],
            ),
            ContractError::OnlyKeyRegisterAdminCanRegisterKeyOnBehalfOfUser {},
        );
    }

    #[test]
    fn cw_friend_contract_multi_test_key_register_admin_can_register_key_on_behalf_of_user() {
        let (mut app, cw_friend_contract_addr, _, key_register_admin_addr, _, user_1_addr, _) =
            proper_instantiate();

        app.execute_contract(
            user_1_addr.clone(),
            cw_friend_contract_addr.clone(),
            &ExecuteMsg::Register(),
            &[],
        )
        .unwrap();

        app.execute_contract(
            key_register_admin_addr.clone(),
            cw_friend_contract_addr.clone(),
            &ExecuteMsg::RegisterSocialMediaAndKey(RegisterSocialMediaAndKeyMsg {
                user_addr: user_1_addr.clone(),
                social_media_handle: SOCIAL_MEDIA_HANDLE_1.to_string(),
            }),
            &[],
        )
        .unwrap();

        let query_user_1_res: UserResponse = app
            .wrap()
            .query_wasm_smart(
                cw_friend_contract_addr.clone(),
                &QueryMsg::QueryUser(QueryUserMsg {
                    user_addr: user_1_addr.clone(),
                }),
            )
            .unwrap();
        assert_eq!(
            query_user_1_res,
            UserResponse {
                user: User {
                    addr: user_1_addr.clone(),
                    social_media_handle: Some(SOCIAL_MEDIA_HANDLE_1.to_string()),
                    issued_key: Some(Key {
                        supply: Uint128::from(1 as u8),
                    })
                }
            }
        );

        let query_user_1_key_supply_res: KeySupplyResponse = app
            .wrap()
            .query_wasm_smart(
                cw_friend_contract_addr.clone(),
                &QueryMsg::QueryKeySupply(QueryKeySupplyMsg {
                    key_issuer_addr: user_1_addr.clone(),
                }),
            )
            .unwrap();
        assert_eq!(
            query_user_1_key_supply_res,
            KeySupplyResponse {
                supply: Uint128::from(1 as u8),
            }
        );

        let query_user_1_holdings_res: UserHoldingsResponse = app
            .wrap()
            .query_wasm_smart(
                cw_friend_contract_addr.clone(),
                &QueryMsg::QueryUserHoldings(QueryUserHoldingsMsg {
                    user_addr: user_1_addr.clone(),
                    start_after_key_issuer_addr: None,
                    limit: None,
                }),
            )
            .unwrap();
        assert_eq!(
            query_user_1_holdings_res,
            UserHoldingsResponse {
                user_holdings: vec![UserHolding {
                    issuer_addr: user_1_addr.clone(),
                    amount: Uint128::from(1 as u8)
                }],
                total_count: 1
            }
        );

        let query_user_1_key_holders_res: KeyHoldersResponse = app
            .wrap()
            .query_wasm_smart(
                cw_friend_contract_addr.clone(),
                &QueryMsg::QueryKeyHolders(QueryKeyHoldersMsg {
                    key_issuer_addr: user_1_addr.clone(),
                    start_after_user_addr: None,
                    limit: None,
                }),
            )
            .unwrap();
        assert_eq!(
            query_user_1_key_holders_res,
            KeyHoldersResponse {
                key_holders: vec![KeyHolder {
                    holder_addr: user_1_addr.clone(),
                    amount: Uint128::from(1 as u8)
                }],
                total_count: 1
            }
        );
    }

    #[test]
    fn cw_friend_contract_multi_test_buy_key() {
        let (
            mut app,
            cw_friend_addr,
            admin_addr,
            key_register_admin_addr,
            fee_collector_addr,
            user_1_addr,
            user_2_addr,
        ) = proper_instantiate();
    }

    #[test]
    fn cw_friend_contract_multi_test_sell_key() {
        let (
            mut app,
            cw_friend_addr,
            admin_addr,
            key_register_admin_addr,
            fee_collector_addr,
            user_1_addr,
            user_2_addr,
        ) = proper_instantiate();
    }
}
