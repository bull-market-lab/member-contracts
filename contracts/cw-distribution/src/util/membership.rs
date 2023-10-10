use cosmwasm_std::{Addr, Deps, Uint128, Uint64};

use membership::{
    config::Config,
    msg::{
        ConfigResponse, MembershipsResponse, QueryConfigMsg, QueryMembersMsg, QueryMsg,
        QueryUserByIDMsg, UserResponse,
    },
    user::User,
};

use crate::ContractError;

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

pub fn query_user_membership_amount(
    deps: Deps,
    membership_contract_addr: Addr,
    membership_issuer_user_id: u64,
    user_id: u64,
) -> Result<Uint128, ContractError> {
    let resp: MembershipsResponse = deps
        .querier
        .query_wasm_smart(
            membership_contract_addr,
            &QueryMsg::QueryMembers(QueryMembersMsg {
                membership_issuer_user_id: Uint64::from(membership_issuer_user_id),
                limit: Some(1),
                start_after_member_user_id: Some(Uint64::from(user_id)),
                include_start_after: Some(true),
            }),
        )
        .unwrap();
    if resp.memberships.len() != 1 {
        return Err(ContractError::ErrorGettingUserMembershipResultNotOne {});
    }
    Ok(resp.memberships[0].amount)
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
