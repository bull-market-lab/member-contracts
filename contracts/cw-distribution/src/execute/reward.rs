use cosmwasm_std::{Addr, Decimal, DepsMut, MessageInfo, Response, Uint128};

use distribution::msg::{
    DistributeMsg, SetupDistributionForNewMemberMsg, SetupDistributionForNewMembershipMsg,
};

use crate::{
    state::{GLOBAL_INDICES, USERS_DISTRIBUTIONS},
    ContractError,
};

pub fn setup_distribution_for_new_membership(
    deps: DepsMut,
    info: MessageInfo,
    data: SetupDistributionForNewMembershipMsg,
    membership_contract_addr: Addr,
) -> Result<Response, ContractError> {
    if info.sender != membership_contract_addr {
        return Err(ContractError::OnlyMembershipContractCanSetupDistributionForNewMembership {});
    }

    let membership_issuer_user_id = data.membership_issuer_user_id.u64();
    let global_index = Decimal::zero();
    let pending_reward = Uint128::zero();

    GLOBAL_INDICES.update(
        deps.storage,
        membership_issuer_user_id,
        |existing| match existing {
            None => Ok(global_index),
            Some(_) => Err(ContractError::GlobalIndicesAlreadySetupForMembershipIssuer {}),
        },
    )?;

    USERS_DISTRIBUTIONS.update(
        deps.storage,
        (membership_issuer_user_id, membership_issuer_user_id),
        |existing| match existing {
            None => Ok((global_index, pending_reward)),
            Some(_) => Err(ContractError::DistributionAlreadySetupForMembershipIssuer {}),
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "setup_distribution_for_new_membership")
        .add_attribute("membership_issuer_user_id", data.membership_issuer_user_id))
}

pub fn setup_distribution_for_new_member(
    deps: DepsMut,
    info: MessageInfo,
    data: SetupDistributionForNewMemberMsg,
    membership_contract_addr: Addr,
) -> Result<Response, ContractError> {
    if info.sender != membership_contract_addr {
        return Err(ContractError::OnlyMembershipContractCanSetupDistributionForNewMember {});
    }

    let membership_issuer_user_id = data.membership_issuer_user_id.u64();
    let user_id = data.user_id.u64();
    let global_index = GLOBAL_INDICES.load(deps.storage, membership_issuer_user_id)?;
    let pending_reward = Uint128::zero();

    USERS_DISTRIBUTIONS.update(
        deps.storage,
        (membership_issuer_user_id, user_id),
        |existing| match existing {
            None => Ok((global_index, pending_reward)),
            Some(_) => Err(ContractError::DistributionAlreadySetupForMembershipIssuer {}),
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "setup_distribution_for_new_member")
        .add_attribute("membership_issuer_user_id", data.membership_issuer_user_id)
        .add_attribute("user_id", data.user_id))
}

/// Distributes new rewards for a membership program, using funds found in MessageInfo.
/// Will increase global index for each of the assets being distributed.
pub fn distribute(
    deps: DepsMut,
    info: MessageInfo,
    data: DistributeMsg,
    membership_contract_addr: Addr,
) -> Result<Response, ContractError> {
    if info.sender != membership_contract_addr {
        return Err(ContractError::OnlyMembershipContractCanDistribute {});
    }

    let membership_issuer_user_id = data.membership_issuer_user_id.u64();

    GLOBAL_INDICES.update(
        deps.storage,
        membership_issuer_user_id,
        |index| match index {
            None => Err(ContractError::CannotDistributeBeforeSetupDistribution {}),
            Some(index) => Ok(index + data.index_increment),
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "distribute")
        .add_attribute("membership_issuer_user_id", data.membership_issuer_user_id))
}
