use cosmwasm_std::{Addr, Deps, Uint64};
use membership::{
    config::Config,
    msg::{ConfigResponse, QueryConfigMsg, QueryMsg, QueryUserByIDMsg, UserResponse},
    user::User,
};

pub fn query_membership_contract_config(deps: Deps, membership_contract_addr: Addr) -> Config {
    let resp: ConfigResponse = deps
        .querier
        .query_wasm_smart(
            membership_contract_addr,
            &QueryMsg::QueryConfig(QueryConfigMsg {}),
        )
        .unwrap();
    resp.config
}

pub fn query_user(deps: Deps, membership_contract_addr: Addr, user_id: u64) -> User {
    let resp: UserResponse = deps
        .querier
        .query_wasm_smart(
            membership_contract_addr,
            &QueryMsg::QueryUserByID(QueryUserByIDMsg {
                user_id: Uint64::from(user_id),
            }),
        )
        .unwrap();
    resp.user
}
