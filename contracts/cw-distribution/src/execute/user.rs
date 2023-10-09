use cosmwasm_std::{DepsMut, MessageInfo, Response, Uint64};

use distribution::{config::Config, distribution::Distribution, msg::UpdateUserWeightsMsg};

use crate::{
    state::{GLOBAL_INDICES_AND_EFFECTIVE_TOTAL_WEIGHT, USERS_DISTRIBUTIONS},
    util::user::initialize_user_indices,
    ContractError,
};

pub fn update_user_weight(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateUserWeightsMsg,
    config: Config,
) -> Result<Response, ContractError> {
    if info.sender != config.admin_addr {
        return Err(ContractError::OnlyAdminCanUpdateUserWeight {});
    }

    let membership_issuer_user_id = data.membership_issuer_user_id.u64();
    let user_id = data.user_id.u64();

    let previous_effective_total_weight = GLOBAL_INDICES_AND_EFFECTIVE_TOTAL_WEIGHT
        .load(deps.storage, membership_issuer_user_id)?
        .1;

    let minimum_eligible_weight = config.minimum_eligible_weight;

    // let user = ctx.deps.api.addr_validate(&user_weight_change.user)?;

    // USERS_DISTRIBUTIONS.update(
    //     deps.storage,
    //     (membership_issuer_user_id, user_id),
    //     |distribution| match distribution {
    //         None => Err(ContractError::UserNotExist {}),
    //         Some(mut user) => {
    //             user.membership_issued_by_me
    //                 .as_mut()
    //                 .unwrap()
    //                 .membership_supply += data.amount;
    //             Ok(user)
    //         }
    //     },
    // );

    let current_global_index = GLOBAL_INDICES_AND_EFFECTIVE_TOTAL_WEIGHT
        .load(deps.storage, membership_issuer_user_id)?
        .0;

    match USERS_DISTRIBUTIONS.may_load(deps.storage, (membership_issuer_user_id, user_id))? {
        None => USERS_DISTRIBUTIONS.save(
            deps.storage,
            (membership_issuer_user_id, user_id),
            &Distribution {
                user_id: data.user_id,
                user_index: current_global_index,
                pending_rewards: todo!(),
                real_weight: todo!(),
                effective_weight: todo!(),
            },
        )?,
        Some(mut user) => {
            // user.membership_issued_by_me
            //     .as_mut()
            //     .unwrap()
            //     .membership_supply += data.amount;
            // Ok(user)
        }
    }

    let old_user_distribution =
        USERS_DISTRIBUTIONS.may_load(deps.storage, (membership_issuer_user_id, user_id))?;

    match old_user_distribution {
        None => {
            // we have not encountered this user, so we need to ensure their distribution
            // indices are set to current global indices
            initialize_user_indices(deps.storage, user.clone())?;
        }
        Some(old_user_effective_weight) => {
            // the user already had their weight previously, so we use that weight
            // to calculate how many rewards for each asset they've accrued since we last
            // calculated their pending rewards
            update_user_distributions(ctx.deps.branch(), user.clone(), old_user_effective_weight)?;
        }
    };

    USER_WEIGHTS.save(ctx.deps.storage, user.clone(), &user_weight_change.weight)?;

    let effective_user_weight =
        calculate_effective_weight(user_weight_change.weight, minimum_eligible_weight);
    EFFECTIVE_USER_WEIGHTS.save(ctx.deps.storage, user, &effective_user_weight)?;

    let old_user_effective_weight = old_user_effective_weight.unwrap_or_default();

    effective_total_weight =
        effective_total_weight - old_user_effective_weight + effective_user_weight;

    GLOBAL_INDICES_AND_EFFECTIVE_TOTAL_WEIGHT.save(ctx.deps.storage, &effective_total_weight)?;

    Ok(Response::new()
        .add_attribute("action", "update_user_weight")
        .add_attribute("user_id", user_id))
}
