use cosmwasm_std::{Decimal, DepsMut, MessageInfo, Response, Uint128};

use distribution::{
    config::Config,
    distribution::Distribution,
    msg::{ClaimRewardsMsg, SetupDistributionForNewMembershipMsg},
};

use crate::{
    state::{GLOBAL_INDICES_AND_TOTAL_WEIGHT, USERS_DISTRIBUTIONS},
    ContractError,
};

pub fn setup_distribution_for_new_membership(
    deps: DepsMut,
    info: MessageInfo,
    data: SetupDistributionForNewMembershipMsg,
    config: Config,
) -> Result<Response, ContractError> {
    if info.sender != config.membership_contract_addr {
        return Err(ContractError::OnlyAdminCanSetupDistributionForNewMembership {});
    }

    let membership_issuer_user_id = data.membership_issuer_user_id.u64();

    // Initial weight for new user is 1, since supply is 1
    // TODO: should we even store weight in distribution contract?
    // Can't we just read the user holding balance from membership contract?
    let real_user_weight = Uint128::one();

    // TODO: P2: decide if we need minimum eligible weight
    // let effective_user_weight =
    //     calculate_effective_weight(real_user_weight, config.minimum_eligible_weight);
    let total_user_weight = real_user_weight;

    GLOBAL_INDICES_AND_TOTAL_WEIGHT.update(
        deps.storage,
        membership_issuer_user_id,
        |existing| match existing {
            None => Ok((Decimal::zero(), real_user_weight)),
            Some(_) => Err(ContractError::GlobalIndicesAndEffectiveTotalWeightAlreadySetupForMembershipIssuer {}),
        },
    )?;

    USERS_DISTRIBUTIONS.update(
        deps.storage,
        (membership_issuer_user_id, membership_issuer_user_id),
        |existing| match existing {
            None => Ok(Distribution {
                user_id: data.membership_issuer_user_id,
                user_index: Decimal::zero(),
                pending_rewards: Uint128::zero(),
                real_weight: real_user_weight,
                // effective_weight: effective_user_weight,
            }),
            Some(_) => Err(ContractError::DistributionAlreadySetupForMembershipIssuer {}),
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "setup_distribution_for_new_membership")
        .add_attribute("user_id", data.membership_issuer_user_id))
}

/// Distributes new rewards for a native asset, using funds found in MessageInfo.
/// Will increase global index for each of the assets being distributed.
pub fn distribute(
    deps: DepsMut,
    info: MessageInfo,
    config: Config,
    distributed_amount: Uint128,
) -> Result<Response, ContractError> {
    // let funds = ctx.info.funds.clone();

    // let distribution_assets = funds
    //     .iter()
    //     .map(|coin| AssetInfo::native(coin.denom.to_string()))
    //     .collect();
    // assert_assets_whitelisted(ctx, distribution_assets)?;

    let total_weight = EFFECTIVE_TOTAL_WEIGHT.load(ctx.deps.storage)?;
    if total_weight == Uint128::zero() {
        return Err(ZeroTotalWeight);
    }

    for fund in funds {
        let global_index = NATIVE_GLOBAL_INDICES
            .may_load(ctx.deps.storage, fund.denom.clone())?
            .unwrap_or(Decimal::zero());

        // calculate how many units of the asset we're distributing per unit of total user weight
        // and add that to the global index for the asset
        let index_increment = Decimal::from_ratio(fund.amount, total_weight);

        NATIVE_GLOBAL_INDICES.save(
            ctx.deps.storage,
            fund.denom,
            &global_index.checked_add(index_increment)?,
        )?;
    }

    Ok(execute_distribute_native_response(total_weight))
}

pub fn claim_reward(
    deps: DepsMut,
    info: MessageInfo,
    data: ClaimRewardsMsg,
    config: Config,
) -> Result<Response, ContractError> {
    if info.sender != config.registration_admin_addr {
        return Err(ContractError::OnlyRegistrationAdminCanLinkSocialMediaOnBehalfOfUser {});
    }

    let user = USERS()
        .idx
        .id
        .item(deps.storage, data.user_id.u64())?
        .unwrap()
        .1;

    USERS().update(deps.storage, &user.addr, |user| match user {
        // User should exist in USERS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            // User should not have linked a social media handle yet
            if user.social_media_handle.is_some() {
                return Err(ContractError::UserAlreadyLinkedSocialMedia {});
            }
            let updated_user = User {
                id: user.id,
                addr: user.addr,
                social_media_handle: Some(data.social_media_handle.clone()),
                membership_issued_by_me: user.membership_issued_by_me,
                trading_fee_percentage_of_membership: user.trading_fee_percentage_of_membership,
                share_to_issuer_percentage: user.share_to_issuer_percentage,
                share_to_all_members_percentage: user.share_to_all_members_percentage,
                user_member_count: user.user_member_count,
            };
            Ok(updated_user)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "link_social_media")
        .add_attribute("user_id", user.id)
        .add_attribute("user_addr", user.addr)
        .add_attribute("social_media_handle", data.social_media_handle))
}
