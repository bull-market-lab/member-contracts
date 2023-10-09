use cosmwasm_std::{Decimal, DepsMut, Order, StdResult};

/// Called for users that we did not encounter previously.
///
/// Will initialize all their rewards for assets with existing distributions to 0, and set
/// their rewards indices to current global index for each asset.
pub fn initialize_user_indices(deps: DepsMut, membership_issuer_user_id: u64, user_id: u64) {
    let native_global_indices = GLOBAL_INDICES
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<(u64, Decimal)>>>()
        .unwrap();

    for (denom, global_index) in native_global_indices {
        DISTRIBUTIONS().update(
            ctx.deps.storage,
            (user.clone(), denom.clone()),
            |distribution| -> StdResult<NativeDistribution> {
                match distribution {
                    None => Ok(NativeDistribution {
                        user: user.clone(),
                        denom,
                        user_index: global_index,
                        pending_rewards: Uint128::zero(),
                    }),
                    Some(distribution) => Ok(distribution),
                }
            },
        )?;
    }
}

/// Updates user's reward indices for all native assets.
///
/// Will calculate newly pending rewards since the last update to the user's reward index until now,
/// using their last weight to calculate the newly accrued rewards.
pub fn update_user_native_distributions(
    deps: DepsMut,
    membership_issuer_user_id: u64,
    user_id: u64,
) {
    let native_global_indices = NATIVE_GLOBAL_INDICES
        .range(deps.storage, None, None, Ascending)
        .collect::<StdResult<Vec<(String, Decimal)>>>()?;

    for (denom, global_index) in native_global_indices {
        let distribution =
            NATIVE_DISTRIBUTIONS().may_load(deps.storage, (user.clone(), denom.clone()))?;

        let reward = calculate_user_reward(global_index, distribution, old_user_weight)?;

        NATIVE_DISTRIBUTIONS().save(
            deps.storage,
            (user.clone(), denom.clone()),
            &NativeDistribution {
                user: user.clone(),
                denom,
                user_index: global_index,
                pending_rewards: reward,
            },
        )?;
    }
}
