use cosmwasm_std::{Addr, Deps, Fraction, StdResult};

use distribution_pkg::msg::{QueryUserRewardMsg, UserRewardResponse};
use member_pkg::member_contract_querier::query_is_user_a_member_and_membership_amount;

use crate::state::{ALL_USERS_DISTRIBUTIONS, GLOBAL_INDICES};

pub fn query_user_reward(
    deps: Deps,
    data: QueryUserRewardMsg,
    membership_contract_addr: Addr,
) -> StdResult<UserRewardResponse> {
    let membership_issuer_user_id = data.membership_issuer_user_id.u64();
    let user_id = data.user_id.u64();

    let global_index = GLOBAL_INDICES.load(deps.storage, membership_issuer_user_id)?;
    let (user_index, pending_reward) =
        ALL_USERS_DISTRIBUTIONS.load(deps.storage, (membership_issuer_user_id, user_id))?;

    // Query membership contract for user membership amount
    let (_, user_amount) = query_is_user_a_member_and_membership_amount(
        deps,
        membership_contract_addr,
        membership_issuer_user_id,
        user_id,
    );

    let user_index_diff = global_index.checked_sub(user_index).unwrap();
    let new_reward = user_amount
        .checked_multiply_ratio(user_index_diff.numerator(), user_index_diff.denominator())
        .unwrap();

    Ok(UserRewardResponse {
        amount: new_reward + pending_reward,
    })
}
