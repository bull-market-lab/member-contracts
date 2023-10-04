#[cfg(test)]
mod tests {
    use anyhow::Result as AnyResult;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, AppResponse, Contract, ContractWrapper, Executor};

    use thread::{
        key_holder::KeyHolder,
        msg::{
            BuyKeyMsg, CostToBuyKeyResponse, CostToSellKeyResponse, ExecuteMsg, InstantiateMsg,
            KeyHoldersResponse, KeySupplyResponse, LinkSocialMediaMsg, QueryCostToBuyKeyMsg,
            QueryCostToSellKeyMsg, QueryKeyHoldersMsg, QueryKeySupplyMsg, QueryMsg,
            QueryUserHoldingsMsg, QueryUserMsg, RegisterKeyMsg, SellKeyMsg, UserHoldingsResponse,
            UserResponse,
        },
        user::User,
        user_holding::UserHolding,
    };

    use crate::{
        contract::{execute, instantiate, query},
        ContractError,
    };

    const FAUCET: &str = "faucet";

    const ADMIN: &str = "terra1";
    const REGISTER_ADMIN: &str = "terra2";
    const FEE_COLLECTOR: &str = "terra3";

    const USER_1: &str = "terra4";
    const USER_2: &str = "terra5";

    const SOCIAL_MEDIA_HANDLE_1: &str = "twitter1";
    // const SOCIAL_MEDIA_HANDLE_2: &str = "twitter2";

    const FEE_DENOM: &str = "uluna";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(FAUCET),
                    vec![Coin {
                        denom: FEE_DENOM.to_string(),
                        // 1_000_000_000 uLuna i.e. 1k LUNA since 1 LUNA = 1_000_000 uLuna
                        amount: Uint128::new(1_000_000_000),
                    }],
                )
                .unwrap();
        })
    }

    fn contract_cw_thread() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    fn proper_instantiate() -> (App, Addr, Addr, Addr, Addr, Addr, Addr) {
        let mut app = mock_app();
        let cw_thread_contract_code_id = app.store_code(contract_cw_thread());

        let msg = InstantiateMsg {
            admin_addr: Some(ADMIN.to_string()),
            registration_admin_addr: Some(REGISTER_ADMIN.to_string()),
            protocol_fee_collector_addr: Some(FEE_COLLECTOR.to_string()),
            fee_denom: Some(FEE_DENOM.to_string()),
            // Use default value for all below
            max_thread_title_length: None,
            max_thread_description_length: None,
            max_thread_label_length: None,
            max_number_of_thread_labels: None,
            max_thread_msg_length: None,
            protocol_fee_key_trading_fee_percentage: None,
            protocol_fee_start_new_thread_fixed_cost: None,
            protocol_fee_ask_in_thread_fee_percentage: None,
            protocol_fee_reply_in_thread_fee_percentage: None,
            default_trading_fee_percentage_of_key: None,
            default_ask_fee_percentage_of_key: None,
            default_reply_fee_percentage_of_key: None,
            default_key_trading_fee_key_issuer_fee_percentage: None,
            default_key_trading_fee_key_holder_fee_percentage: None,
            default_thread_fee_key_issuer_fee_percentage: None,
            default_thread_fee_key_holder_fee_percentage: None,
        };
        let cw_thread_contract_addr = app
            .instantiate_contract(
                cw_thread_contract_code_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "cw_thread",
                None,
            )
            .unwrap();

        let admin_addr = Addr::unchecked(ADMIN.to_string());
        let registration_admin_addr = Addr::unchecked(REGISTER_ADMIN.to_string());
        let fee_collector_addr = Addr::unchecked(FEE_COLLECTOR.to_string());
        let user_1_addr = Addr::unchecked(USER_1.to_string());
        let user_2_addr = Addr::unchecked(USER_2.to_string());

        (
            app,
            cw_thread_contract_addr,
            admin_addr,
            registration_admin_addr,
            fee_collector_addr,
            user_1_addr,
            user_2_addr,
        )
    }

    fn get_fund_from_faucet(app: &mut App, addr: Addr, amount: Uint128) {
        app.send_tokens(
            Addr::unchecked(FAUCET),
            addr,
            &[Coin {
                denom: FEE_DENOM.to_string(),
                amount,
            }],
        )
        .unwrap();
    }

    fn register_user(app: &mut App, cw_thread_contract_addr: &Addr, user_addr: &Addr) {
        app.execute_contract(
            user_addr.clone(),
            cw_thread_contract_addr.clone(),
            &ExecuteMsg::Register(),
            &[],
        )
        .unwrap();
    }

    fn link_social_media_and_register_key(
        app: &mut App,
        cw_thread_contract_addr: &Addr,
        registration_admin_addr: &Addr,
        user_addr: &Addr,
        social_media_handle: &str,
    ) {
        app.execute_contract(
            registration_admin_addr.clone(),
            cw_thread_contract_addr.clone(),
            &ExecuteMsg::LinkSocialMedia(LinkSocialMediaMsg {
                user_addr: user_addr.to_string(),
                social_media_handle: social_media_handle.to_string(),
            }),
            &[],
        )
        .unwrap();
        app.execute_contract(
            registration_admin_addr.clone(),
            cw_thread_contract_addr.clone(),
            &ExecuteMsg::RegisterKey(RegisterKeyMsg {
                user_addr: user_addr.to_string(),
            }),
            &[],
        )
        .unwrap();
    }

    fn print_balance(
        app: &App,
        contract_addr: &Addr,
        admin_addr: &Addr,
        fee_collector_addr: &Addr,
        registration_admin_addr: &Addr,
        user_1_addr: &Addr,
        user_2_addr: &Addr,
    ) {
        println!(
            "contract_addr balance {}, admin balance {}, fee_collector balance {}, register_admin balance {}, user_1 balance {}, user_2 balance {}",
            app.wrap().query_balance(contract_addr.clone(), FEE_DENOM).unwrap(),
            app.wrap().query_balance(admin_addr.clone(), FEE_DENOM).unwrap(),
            app.wrap().query_balance(fee_collector_addr.clone(), FEE_DENOM).unwrap(),
            app.wrap().query_balance(registration_admin_addr.clone(), FEE_DENOM).unwrap(),
            app.wrap().query_balance(user_1_addr.clone(), FEE_DENOM).unwrap(),
            app.wrap().query_balance(user_2_addr.clone(), FEE_DENOM).unwrap(),
        );
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

    fn assert_balance(app: &App, user_addr: &Addr, expected_balance: Uint128, denom: &str) {
        let balance = app.wrap().query_balance(user_addr, denom).unwrap();
        assert_eq!(balance.amount, expected_balance);
    }

    fn assert_key_supply(
        app: &App,
        contract_addr: &Addr,
        key_issuer_addr: &Addr,
        expected_supply: Uint128,
    ) {
        let query_key_supply_res: KeySupplyResponse = app
            .wrap()
            .query_wasm_smart(
                contract_addr,
                &QueryMsg::QueryKeySupply(QueryKeySupplyMsg {
                    key_issuer_addr: key_issuer_addr.to_string(),
                }),
            )
            .unwrap();
        assert_eq!(
            query_key_supply_res,
            KeySupplyResponse {
                supply: expected_supply
            }
        );
    }

    fn assert_key_holders(
        app: &App,
        contract_addr: &Addr,
        key_issuer_addr: &Addr,
        expected_key_holders: Vec<KeyHolder>,
    ) {
        let query_key_holders_res: KeyHoldersResponse = app
            .wrap()
            .query_wasm_smart(
                contract_addr,
                &QueryMsg::QueryKeyHolders(QueryKeyHoldersMsg {
                    key_issuer_addr: key_issuer_addr.to_string(),
                    start_after_user_addr: None,
                    limit: None,
                }),
            )
            .unwrap();
        assert_eq!(
            query_key_holders_res,
            KeyHoldersResponse {
                key_holders: expected_key_holders.clone(),
                total_count: expected_key_holders.len() as usize,
                count: expected_key_holders.len() as usize
            }
        );
    }

    fn assert_user_holdings(
        app: &App,
        contract_addr: &Addr,
        user_addr: &Addr,
        expected_user_holdings: Vec<UserHolding>,
    ) {
        let query_user_holdings_res: UserHoldingsResponse = app
            .wrap()
            .query_wasm_smart(
                contract_addr,
                &QueryMsg::QueryUserHoldings(QueryUserHoldingsMsg {
                    user_addr: user_addr.to_string(),
                    start_after_key_issuer_addr: None,
                    limit: None,
                }),
            )
            .unwrap();
        assert_eq!(
            query_user_holdings_res,
            UserHoldingsResponse {
                user_holdings: expected_user_holdings.clone(),
                total_count: expected_user_holdings.len() as usize,
                count: expected_user_holdings.len() as usize
            }
        );
    }

    #[test]
    fn cw_thread_contract_multi_test_user_can_register_itself() {
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
                    reply_fee_percentage_of_key: None,
                    key_trading_fee_share_config: None,
                    thread_fee_share_config: None
                }
            }
        );
    }

    #[test]
    fn cw_thread_contract_multi_test_user_cannot_register_key_by_itself() {
        let (mut app, cw_thread_contract_addr, _, _, _, user_1_addr, _) = proper_instantiate();
        register_user(&mut app, &cw_thread_contract_addr, &user_1_addr);
        assert_err(
            app.execute_contract(
                user_1_addr.clone(),
                cw_thread_contract_addr.clone(),
                &ExecuteMsg::LinkSocialMedia(LinkSocialMediaMsg {
                    user_addr: user_1_addr.to_string(),
                    social_media_handle: SOCIAL_MEDIA_HANDLE_1.to_string(),
                }),
                &[],
            ),
            ContractError::OnlyRegistrationAdminCanLinkSocialMediaOnBehalfOfUser {},
        );
        assert_err(
            app.execute_contract(
                user_1_addr.clone(),
                cw_thread_contract_addr.clone(),
                &ExecuteMsg::RegisterKey(RegisterKeyMsg {
                    user_addr: user_1_addr.to_string(),
                }),
                &[],
            ),
            ContractError::OnlyRegistrationAdminCanRegisterKeyOnBehalfOfUser {},
        );
    }

    #[test]
    fn cw_thread_contract_multi_test_register_admin_can_register_key_on_behalf_of_user() {
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

    #[test]
    fn cw_thread_contract_multi_test_buy_key() {
        let (
            mut app,
            cw_thread_contract_addr,
            admin_addr,
            registration_admin_addr,
            fee_collector_addr,
            user_1_addr,
            user_2_addr,
        ) = proper_instantiate();

        let default_supply: Uint128 = Uint128::one();
        let uint_128_amount_30: Uint128 = Uint128::from(30_u8);
        let uint_128_amount_20: Uint128 = Uint128::from(20_u8);

        register_user(&mut app, &cw_thread_contract_addr, &user_1_addr);
        link_social_media_and_register_key(
            &mut app,
            &cw_thread_contract_addr,
            &registration_admin_addr,
            &user_1_addr,
            SOCIAL_MEDIA_HANDLE_1,
        );

        assert_key_supply(&app, &cw_thread_contract_addr, &user_1_addr, Uint128::one());

        print_balance(
            &app,
            &cw_thread_contract_addr,
            &admin_addr,
            &fee_collector_addr,
            &registration_admin_addr,
            &user_1_addr,
            &user_2_addr,
        );

        // User 1 buy 30 amount of its own keys
        let query_user_1_simulate_buy_key_res: CostToBuyKeyResponse = app
            .wrap()
            .query_wasm_smart(
                cw_thread_contract_addr.clone(),
                &QueryMsg::QueryCostToBuyKey(QueryCostToBuyKeyMsg {
                    key_issuer_addr: user_1_addr.to_string(),
                    amount: uint_128_amount_30,
                }),
            )
            .unwrap();
        assert_eq!(
            query_user_1_simulate_buy_key_res,
            CostToBuyKeyResponse {
                price: Uint128::from(590_937_u32),
                protocol_fee: Uint128::from(2954_u32),
                key_issuer_fee: Uint128::from(14_773_u32),
                key_holder_fee: Uint128::from(14_773_u32),
                total_needed_from_user: Uint128::from(623_437_u32),
            }
        );
        get_fund_from_faucet(&mut app, user_1_addr.clone(), Uint128::one());
        assert_err(
            app.execute_contract(
                user_1_addr.clone(),
                cw_thread_contract_addr.clone(),
                &ExecuteMsg::BuyKey(BuyKeyMsg {
                    key_issuer_addr: user_1_addr.to_string(),
                    amount: uint_128_amount_30,
                }),
                &[Coin {
                    denom: FEE_DENOM.to_string(),
                    amount: Uint128::one(),
                }],
            ),
            ContractError::InsufficientFundsToPayDuringBuy {
                needed: Uint128::from(623_437_u32),
                available: Uint128::one(),
            },
        );
        get_fund_from_faucet(
            &mut app,
            user_1_addr.clone(),
            query_user_1_simulate_buy_key_res.total_needed_from_user - Uint128::one(),
        );
        app.execute_contract(
            user_1_addr.clone(),
            cw_thread_contract_addr.clone(),
            &ExecuteMsg::BuyKey(BuyKeyMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_30,
            }),
            &[Coin {
                denom: FEE_DENOM.to_string(),
                amount: query_user_1_simulate_buy_key_res.total_needed_from_user,
            }],
        )
        .unwrap();
        print_balance(
            &app,
            &cw_thread_contract_addr,
            &admin_addr,
            &fee_collector_addr,
            &registration_admin_addr,
            &user_1_addr,
            &user_2_addr,
        );
        assert_balance(
            &app,
            &cw_thread_contract_addr,
            query_user_1_simulate_buy_key_res.price,
            FEE_DENOM,
        );
        assert_balance(
            &app,
            &user_1_addr,
            query_user_1_simulate_buy_key_res.key_issuer_fee
                + query_user_1_simulate_buy_key_res.key_holder_fee,
            FEE_DENOM,
        );
        assert_balance(
            &app,
            &fee_collector_addr,
            query_user_1_simulate_buy_key_res.protocol_fee,
            FEE_DENOM,
        );
        assert_key_supply(
            &app,
            &cw_thread_contract_addr,
            &user_1_addr,
            default_supply + uint_128_amount_30,
        );
        assert_user_holdings(
            &app,
            &cw_thread_contract_addr,
            &user_1_addr,
            vec![UserHolding {
                issuer_addr: user_1_addr.clone(),
                amount: default_supply + uint_128_amount_30,
            }],
        );
        assert_key_holders(
            &app,
            &cw_thread_contract_addr,
            &user_1_addr,
            vec![KeyHolder {
                holder_addr: user_1_addr.clone(),
                amount: default_supply + uint_128_amount_30,
            }],
        );

        // User 2 buy 20 amount of user 1's keys
        let query_user_2_simulate_buy_key_res: CostToBuyKeyResponse = app
            .wrap()
            .query_wasm_smart(
                cw_thread_contract_addr.clone(),
                &QueryMsg::QueryCostToBuyKey(QueryCostToBuyKeyMsg {
                    key_issuer_addr: user_1_addr.to_string(),
                    amount: uint_128_amount_20,
                }),
            )
            .unwrap();
        assert_eq!(
            query_user_2_simulate_buy_key_res,
            CostToBuyKeyResponse {
                price: Uint128::from(2_091_875_u32),
                protocol_fee: Uint128::from(10_459_u32),
                key_issuer_fee: Uint128::from(52_296_u32),
                key_holder_fee: Uint128::from(52_296_u32),
                total_needed_from_user: Uint128::from(2_206_926_u32),
            }
        );
        get_fund_from_faucet(
            &mut app,
            user_2_addr.clone(),
            query_user_2_simulate_buy_key_res.total_needed_from_user,
        );
        app.execute_contract(
            user_2_addr.clone(),
            cw_thread_contract_addr.clone(),
            &ExecuteMsg::BuyKey(BuyKeyMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_20,
            }),
            &[Coin {
                denom: FEE_DENOM.to_string(),
                amount: query_user_2_simulate_buy_key_res.total_needed_from_user,
            }],
        )
        .unwrap();
        assert_balance(
            &app,
            &cw_thread_contract_addr,
            query_user_2_simulate_buy_key_res.price + query_user_1_simulate_buy_key_res.price,
            FEE_DENOM,
        );
        assert_balance(
            &app,
            &user_1_addr,
            query_user_2_simulate_buy_key_res.key_issuer_fee
                + query_user_2_simulate_buy_key_res.key_holder_fee
                + query_user_1_simulate_buy_key_res.key_issuer_fee
                + query_user_1_simulate_buy_key_res.key_holder_fee,
            FEE_DENOM,
        );
        assert_balance(&app, &user_2_addr, Uint128::zero(), FEE_DENOM);
        assert_balance(
            &app,
            &fee_collector_addr,
            query_user_2_simulate_buy_key_res.protocol_fee
                + query_user_1_simulate_buy_key_res.protocol_fee,
            FEE_DENOM,
        );
        assert_key_supply(
            &app,
            &cw_thread_contract_addr,
            &user_1_addr,
            default_supply + uint_128_amount_30 + uint_128_amount_20,
        );
        assert_user_holdings(
            &app,
            &cw_thread_contract_addr,
            &user_1_addr,
            vec![UserHolding {
                issuer_addr: user_1_addr.clone(),
                amount: default_supply + uint_128_amount_30,
            }],
        );
        assert_user_holdings(
            &app,
            &cw_thread_contract_addr,
            &user_2_addr,
            vec![UserHolding {
                issuer_addr: user_1_addr.clone(),
                amount: uint_128_amount_20,
            }],
        );
        assert_key_holders(
            &app,
            &cw_thread_contract_addr,
            &user_1_addr,
            vec![
                KeyHolder {
                    holder_addr: user_1_addr.clone(),
                    amount: default_supply + uint_128_amount_30,
                },
                KeyHolder {
                    holder_addr: user_2_addr.clone(),
                    amount: uint_128_amount_20,
                },
            ],
        );
    }

    #[test]
    fn cw_thread_contract_multi_test_single_user_buy_and_sell_keys() {
        let (
            mut app,
            cw_thread_contract_addr,
            admin_addr,
            registration_admin_addr,
            fee_collector_addr,
            user_1_addr,
            user_2_addr,
        ) = proper_instantiate();

        let default_supply: Uint128 = Uint128::one();
        let uint_128_amount_30: Uint128 = Uint128::from(30_u8);
        let uint_128_amount_10: Uint128 = Uint128::from(10_u8);

        register_user(&mut app, &cw_thread_contract_addr, &user_1_addr);
        link_social_media_and_register_key(
            &mut app,
            &cw_thread_contract_addr,
            &registration_admin_addr,
            &user_1_addr,
            SOCIAL_MEDIA_HANDLE_1,
        );

        print_balance(
            &app,
            &cw_thread_contract_addr,
            &admin_addr,
            &fee_collector_addr,
            &registration_admin_addr,
            &user_1_addr,
            &user_2_addr,
        );

        // User 1 tries to sell 1 amount of its own keys but fails because key supply cannot go to 0
        get_fund_from_faucet(&mut app, user_1_addr.clone(), Uint128::one());
        assert_err(
            app.execute_contract(
                user_1_addr.clone(),
                cw_thread_contract_addr.clone(),
                &ExecuteMsg::SellKey(SellKeyMsg {
                    key_issuer_addr: user_1_addr.to_string(),
                    amount: default_supply,
                }),
                &[Coin {
                    denom: FEE_DENOM.to_string(),
                    amount: Uint128::one(),
                }],
            ),
            ContractError::CannotSellLastKey {
                sell: default_supply,
                total_supply: default_supply,
            },
        );

        // User 1 buy 30 amount of its own keys
        let query_user_1_simulate_buy_key_res: CostToBuyKeyResponse = app
            .wrap()
            .query_wasm_smart(
                cw_thread_contract_addr.clone(),
                &QueryMsg::QueryCostToBuyKey(QueryCostToBuyKeyMsg {
                    key_issuer_addr: user_1_addr.to_string(),
                    amount: uint_128_amount_30,
                }),
            )
            .unwrap();
        println!(
            "query_user_1_simulate_buy_key_res {:?}",
            query_user_1_simulate_buy_key_res
        );
        get_fund_from_faucet(
            &mut app,
            user_1_addr.clone(),
            query_user_1_simulate_buy_key_res.total_needed_from_user - Uint128::one(),
        );
        app.execute_contract(
            user_1_addr.clone(),
            cw_thread_contract_addr.clone(),
            &ExecuteMsg::BuyKey(BuyKeyMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_30,
            }),
            &[Coin {
                denom: FEE_DENOM.to_string(),
                amount: query_user_1_simulate_buy_key_res.total_needed_from_user,
            }],
        )
        .unwrap();
        print_balance(
            &app,
            &cw_thread_contract_addr,
            &admin_addr,
            &fee_collector_addr,
            &registration_admin_addr,
            &user_1_addr,
            &user_2_addr,
        );

        // User 1 tries to sell 10 amount of its own keys but fails because it didn't pay enough protocol fee
        let query_user_1_simulate_sell_key_res: CostToBuyKeyResponse = app
            .wrap()
            .query_wasm_smart(
                cw_thread_contract_addr.clone(),
                &QueryMsg::QueryCostToSellKey(QueryCostToSellKeyMsg {
                    key_issuer_addr: user_1_addr.to_string(),
                    amount: uint_128_amount_10,
                }),
            )
            .unwrap();
        get_fund_from_faucet(&mut app, user_1_addr.clone(), Uint128::one());
        assert_err(
            app.execute_contract(
                user_1_addr.clone(),
                cw_thread_contract_addr.clone(),
                &ExecuteMsg::SellKey(SellKeyMsg {
                    key_issuer_addr: user_1_addr.to_string(),
                    amount: uint_128_amount_10,
                }),
                &[Coin {
                    denom: FEE_DENOM.to_string(),
                    amount: Uint128::one(),
                }],
            ),
            ContractError::InsufficientFundsToPayDuringSell {
                needed: query_user_1_simulate_sell_key_res.total_needed_from_user,
                available: Uint128::one(),
            },
        );

        // User 1 tries to sell 30 amount of its own keys and succeeds
        let query_user_1_simulate_sell_key_res: CostToSellKeyResponse = app
            .wrap()
            .query_wasm_smart(
                cw_thread_contract_addr.clone(),
                &QueryMsg::QueryCostToSellKey(QueryCostToSellKeyMsg {
                    key_issuer_addr: user_1_addr.to_string(),
                    amount: uint_128_amount_30,
                }),
            )
            .unwrap();
        // Price should be the same as buying 30 keys because user 1 is the only user buying / selling so far
        get_fund_from_faucet(
            &mut app,
            user_1_addr.clone(),
            query_user_1_simulate_sell_key_res.total_needed_from_user - Uint128::one(),
        );
        println!(
            "query_user_1_simulate_sell_key_res {:?}",
            query_user_1_simulate_sell_key_res
        );
        app.execute_contract(
            user_1_addr.clone(),
            cw_thread_contract_addr.clone(),
            &ExecuteMsg::SellKey(SellKeyMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_30,
            }),
            &[Coin {
                denom: FEE_DENOM.to_string(),
                amount: query_user_1_simulate_sell_key_res.total_needed_from_user,
            }],
        )
        .unwrap();

        assert_balance(&app, &cw_thread_contract_addr, Uint128::zero(), FEE_DENOM);
        assert_balance(
            &app,
            &user_1_addr,
            query_user_1_simulate_sell_key_res.total_needed_from_user
                - query_user_1_simulate_sell_key_res.protocol_fee
                + query_user_1_simulate_buy_key_res.total_needed_from_user
                - query_user_1_simulate_buy_key_res.protocol_fee,
            FEE_DENOM,
        );
        assert_balance(
            &app,
            &fee_collector_addr,
            query_user_1_simulate_sell_key_res.protocol_fee
                + query_user_1_simulate_buy_key_res.protocol_fee,
            FEE_DENOM,
        );
        assert_key_supply(&app, &cw_thread_contract_addr, &user_1_addr, default_supply);
        assert_user_holdings(
            &app,
            &cw_thread_contract_addr,
            &user_1_addr,
            vec![UserHolding {
                issuer_addr: user_1_addr.clone(),
                amount: default_supply,
            }],
        );
        assert_key_holders(
            &app,
            &cw_thread_contract_addr,
            &user_1_addr,
            vec![KeyHolder {
                holder_addr: user_1_addr.clone(),
                amount: default_supply,
            }],
        );
    }

    #[test]
    fn cw_thread_contract_multi_test_2_users_buy_and_sell_keys() {
        let (
            mut app,
            cw_thread_contract_addr,
            _,
            registration_admin_addr,
            fee_collector_addr,
            user_1_addr,
            user_2_addr,
        ) = proper_instantiate();

        let default_supply: Uint128 = Uint128::one();
        let uint_128_amount_30: Uint128 = Uint128::from(30_u8);
        let uint_128_amount_25: Uint128 = Uint128::from(25_u8);
        let uint_128_amount_15: Uint128 = Uint128::from(15_u8);
        let uint_128_amount_10: Uint128 = Uint128::from(10_u8);

        register_user(&mut app, &cw_thread_contract_addr, &user_1_addr);
        link_social_media_and_register_key(
            &mut app,
            &cw_thread_contract_addr,
            &registration_admin_addr,
            &user_1_addr,
            SOCIAL_MEDIA_HANDLE_1,
        );

        // User 1 buy 30 amount of its own keys
        let query_user_1_simulate_buy_key_res: CostToBuyKeyResponse = app
            .wrap()
            .query_wasm_smart(
                cw_thread_contract_addr.clone(),
                &QueryMsg::QueryCostToBuyKey(QueryCostToBuyKeyMsg {
                    key_issuer_addr: user_1_addr.to_string(),
                    amount: uint_128_amount_30,
                }),
            )
            .unwrap();
        get_fund_from_faucet(
            &mut app,
            user_1_addr.clone(),
            query_user_1_simulate_buy_key_res.total_needed_from_user,
        );
        app.execute_contract(
            user_1_addr.clone(),
            cw_thread_contract_addr.clone(),
            &ExecuteMsg::BuyKey(BuyKeyMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_30,
            }),
            &[Coin {
                denom: FEE_DENOM.to_string(),
                amount: query_user_1_simulate_buy_key_res.total_needed_from_user,
            }],
        )
        .unwrap();

        // User 2 buys 25 amount of user 1's keys
        let query_user_2_simulate_buy_key_res: CostToBuyKeyResponse = app
            .wrap()
            .query_wasm_smart(
                cw_thread_contract_addr.clone(),
                &QueryMsg::QueryCostToBuyKey(QueryCostToBuyKeyMsg {
                    key_issuer_addr: user_1_addr.to_string(),
                    amount: uint_128_amount_25,
                }),
            )
            .unwrap();
        get_fund_from_faucet(
            &mut app,
            user_2_addr.clone(),
            query_user_2_simulate_buy_key_res.total_needed_from_user,
        );
        app.execute_contract(
            user_2_addr.clone(),
            cw_thread_contract_addr.clone(),
            &ExecuteMsg::BuyKey(BuyKeyMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_25,
            }),
            &[Coin {
                denom: FEE_DENOM.to_string(),
                amount: query_user_2_simulate_buy_key_res.total_needed_from_user,
            }],
        )
        .unwrap();

        // User 2 sells 15 amount of user 1's keys
        let query_user_2_simulate_sell_key_res: CostToSellKeyResponse = app
            .wrap()
            .query_wasm_smart(
                cw_thread_contract_addr.clone(),
                &QueryMsg::QueryCostToSellKey(QueryCostToSellKeyMsg {
                    key_issuer_addr: user_1_addr.to_string(),
                    amount: uint_128_amount_15,
                }),
            )
            .unwrap();
        get_fund_from_faucet(
            &mut app,
            user_2_addr.clone(),
            query_user_2_simulate_sell_key_res.total_needed_from_user,
        );
        app.execute_contract(
            user_2_addr.clone(),
            cw_thread_contract_addr.clone(),
            &ExecuteMsg::SellKey(SellKeyMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_15,
            }),
            &[Coin {
                denom: FEE_DENOM.to_string(),
                amount: query_user_2_simulate_sell_key_res.total_needed_from_user,
            }],
        )
        .unwrap();

        // User 1 sells 10 amount of its own keys
        let query_user_1_simulate_sell_key_res: CostToSellKeyResponse = app
            .wrap()
            .query_wasm_smart(
                cw_thread_contract_addr.clone(),
                &QueryMsg::QueryCostToSellKey(QueryCostToSellKeyMsg {
                    key_issuer_addr: user_1_addr.to_string(),
                    amount: uint_128_amount_10,
                }),
            )
            .unwrap();
        get_fund_from_faucet(
            &mut app,
            user_1_addr.clone(),
            query_user_1_simulate_sell_key_res.total_needed_from_user,
        );
        app.execute_contract(
            user_1_addr.clone(),
            cw_thread_contract_addr.clone(),
            &ExecuteMsg::SellKey(SellKeyMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_10,
            }),
            &[Coin {
                denom: FEE_DENOM.to_string(),
                amount: query_user_1_simulate_sell_key_res.total_needed_from_user,
            }],
        )
        .unwrap();

        assert_key_supply(
            &app,
            &cw_thread_contract_addr,
            &user_1_addr,
            default_supply + uint_128_amount_30 + uint_128_amount_25
                - uint_128_amount_15
                - uint_128_amount_10,
        );

        assert_user_holdings(
            &app,
            &cw_thread_contract_addr,
            &user_1_addr,
            vec![UserHolding {
                issuer_addr: user_1_addr.clone(),
                amount: default_supply + uint_128_amount_30 - uint_128_amount_10,
            }],
        );
        assert_user_holdings(
            &app,
            &cw_thread_contract_addr,
            &user_2_addr,
            vec![UserHolding {
                issuer_addr: user_1_addr.clone(),
                amount: uint_128_amount_25 - uint_128_amount_15,
            }],
        );
        assert_key_holders(
            &app,
            &cw_thread_contract_addr,
            &user_1_addr,
            vec![
                KeyHolder {
                    holder_addr: user_1_addr.clone(),
                    amount: default_supply + uint_128_amount_30 - uint_128_amount_10,
                },
                KeyHolder {
                    holder_addr: user_2_addr.clone(),
                    amount: uint_128_amount_25 - uint_128_amount_15,
                },
            ],
        );

        assert_balance(
            &app,
            &fee_collector_addr,
            query_user_1_simulate_buy_key_res.protocol_fee
                + query_user_2_simulate_buy_key_res.protocol_fee
                + query_user_2_simulate_sell_key_res.protocol_fee
                + query_user_1_simulate_sell_key_res.protocol_fee,
            FEE_DENOM,
        );

        assert_balance(
            &app,
            &cw_thread_contract_addr,
            query_user_1_simulate_buy_key_res.price + query_user_2_simulate_buy_key_res.price
                - query_user_2_simulate_sell_key_res.price
                - query_user_1_simulate_sell_key_res.price
                + Uint128::from(2_u8),
            FEE_DENOM,
        );

        assert_balance(
            &app,
            &user_1_addr,
            query_user_1_simulate_buy_key_res.key_issuer_fee
                + query_user_2_simulate_buy_key_res.key_issuer_fee
                + query_user_2_simulate_sell_key_res.key_issuer_fee
                + query_user_1_simulate_sell_key_res.key_issuer_fee
                + query_user_1_simulate_sell_key_res.price,
            FEE_DENOM,
        );

        assert_balance(
            &app,
            &user_2_addr,
            query_user_2_simulate_sell_key_res.price,
            FEE_DENOM,
        );
    }
}
