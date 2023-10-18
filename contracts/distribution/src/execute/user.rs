use cosmwasm_std::{
    Addr, BankMsg, Coin, CosmosMsg, DepsMut, Fraction, MessageInfo, Response, Uint128, Uint64,
};

use distribution_pkg::msg::{
    ClaimRewardsMsg, QueryUserRewardMsg, UpdateUserPendingRewardMsg, UserRewardResponse,
};
use member_pkg::member_contract_querier::query_user_by_id;

use crate::{
    query::user::query_user_reward,
    state::{ALL_USERS_DISTRIBUTIONS, GLOBAL_INDICES},
    ContractError,
};

/// Will calculate any accrued reward since the last update to user's reward.
pub fn update_user_pending_reward(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateUserPendingRewardMsg,
    member_contract_addr: Addr,
) -> Result<Response, ContractError> {
    if info.sender != member_contract_addr {
        return Err(ContractError::OnlyMembershipContractCanUpdateUserPendingReward {});
    }

    let membership_issuer_user_id = data.membership_issuer_user_id.u64();
    let user_id = data.user_id.u64();
    let user_previous_amount = data.user_previous_amount;

    let (previous_user_index, previous_pending_reward) =
        ALL_USERS_DISTRIBUTIONS.load(deps.storage, (membership_issuer_user_id, user_id))?;
    let global_index = GLOBAL_INDICES.load(deps.storage, membership_issuer_user_id)?;
    let new_user_index = global_index;

    let user_index_diff = global_index.checked_sub(previous_user_index).unwrap();
    let new_user_reward = user_previous_amount
        .checked_multiply_ratio(user_index_diff.numerator(), user_index_diff.denominator())
        .unwrap()
        .checked_add(previous_pending_reward)
        .unwrap();

    ALL_USERS_DISTRIBUTIONS.update(
        deps.storage,
        (membership_issuer_user_id, user_id),
        |existing| match existing {
            None => Err(ContractError::CannotUpdatePendingRewardBeforeSetupDistribution {}),
            Some((_, _)) => Ok((new_user_index, new_user_reward)),
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "update_user_pending_reward")
        .add_attribute("user_id", data.user_id))
}

pub fn claim_reward(
    deps: DepsMut,
    data: ClaimRewardsMsg,
    member_contract_addr: Addr,
    fee_denom: &str,
) -> Result<Response, ContractError> {
    let deps_ref = deps.as_ref();

    let membership_issuer_user_id = data.membership_issuer_user_id.u64();
    let user_id = data.user_id.u64();
    let user = query_user_by_id(deps_ref, member_contract_addr.clone(), user_id);

    let global_index = GLOBAL_INDICES.load(deps.storage, membership_issuer_user_id)?;
    let new_user_index = global_index;
    let new_pending_reward = Uint128::zero();

    let resp: UserRewardResponse = query_user_reward(
        deps_ref,
        QueryUserRewardMsg {
            membership_issuer_user_id: Uint64::from(membership_issuer_user_id),
            user_id: Uint64::from(user_id),
        },
        member_contract_addr,
    )?;

    let reward = resp.amount;

    // Bump user index to global index and set user pending reward to 0
    ALL_USERS_DISTRIBUTIONS.update(
        deps.storage,
        (membership_issuer_user_id, user_id),
        |existing| match existing {
            None => Err(ContractError::CannotClaimRewardBeforeSetupDistribution {}),
            Some((_, _)) => Ok((new_user_index, new_pending_reward)),
        },
    )?;

    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: user.addr.to_string(),
        amount: vec![Coin {
            denom: fee_denom.to_string(),
            amount: reward,
        }],
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "claim_reward")
        .add_attribute("user_id", data.user_id)
        .add_attribute("membership_issuer_user_id", data.membership_issuer_user_id))
}
