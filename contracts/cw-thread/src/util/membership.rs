use cosmwasm_std::{Addr, Deps};
use membership::{
    config::Config,
    msg::{ConfigResponse, QueryConfigMsg, QueryMsg},
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
