use cosmwasm_std::{Addr, Deps, Uint128, Uint64};
use member::{
    config::Config,
    msg::{
        ConfigResponse, IsMemberResponse, MembershipSupplyResponse, QueryConfigMsg,
        QueryIsMemberMsg, QueryMembershipSupplyMsg, QueryMsg, QueryUserByAddrMsg, QueryUserByIDMsg,
        UserResponse,
    },
    user::User,
};

pub fn query_member_contract_config(deps: Deps, membership_contract_addr: Addr) -> Config {
    let resp: ConfigResponse = deps
        .querier
        .query_wasm_smart(
            membership_contract_addr,
            &QueryMsg::QueryConfig(QueryConfigMsg {}),
        )
        .unwrap();
    resp.config
}

pub fn query_user_by_id(deps: Deps, membership_contract_addr: Addr, user_id: u64) -> User {
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

pub fn query_user_by_addr(deps: Deps, membership_contract_addr: Addr, user_addr: Addr) -> User {
    let resp: UserResponse = deps
        .querier
        .query_wasm_smart(
            membership_contract_addr,
            &QueryMsg::QueryUserByAddr(QueryUserByAddrMsg {
                user_addr: user_addr.to_string(),
            }),
        )
        .unwrap();
    resp.user
}

pub fn query_is_user_a_member_and_membership_amount(
    deps: Deps,
    membership_contract_addr: Addr,
    membership_issuer_user_id: u64,
    user_id: u64,
) -> (bool, Uint128) {
    let resp: IsMemberResponse = deps
        .querier
        .query_wasm_smart(
            membership_contract_addr,
            &QueryMsg::QueryIsMember(QueryIsMemberMsg {
                membership_issuer_user_id: Uint64::from(membership_issuer_user_id),
                user_id: Uint64::from(user_id),
            }),
        )
        .unwrap();
    (resp.is_member, resp.amount)
}

pub fn query_membership_supply(
    deps: Deps,
    membership_contract_addr: Addr,
    membership_issuer_user_id: u64,
) -> Uint128 {
    let resp: MembershipSupplyResponse = deps
        .querier
        .query_wasm_smart(
            membership_contract_addr,
            &QueryMsg::QueryMembershipSupply(QueryMembershipSupplyMsg {
                membership_issuer_user_id: Uint64::from(membership_issuer_user_id),
            }),
        )
        .unwrap();
    resp.supply
}
