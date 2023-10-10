use cosmwasm_std::{Deps, Uint64};

use crate::{
    state::{ALL_USER_CONFIGS, CONFIG},
    ContractError,
};

pub fn assert_config_fee_share_sum_to_100(deps: Deps) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let share_to_issuer_percentage = config.default_share_to_issuer_percentage;
    let share_to_all_members_percentage = config.default_share_to_all_members_percentage;

    if share_to_issuer_percentage + share_to_all_members_percentage != Uint64::from(100_u64) {
        return Err(ContractError::ThreadFeeSharePercentageMustSumTo100 {});
    }

    Ok(())
}

pub fn assert_user_fee_share_sum_to_100(deps: Deps, user_id: u64) -> Result<(), ContractError> {
    let user_config = ALL_USER_CONFIGS.load(deps.storage, user_id)?;

    let share_to_issuer_percentage = user_config.share_to_issuer_percentage;
    let share_to_all_members_percentage = user_config.share_to_all_members_percentage;

    if share_to_all_members_percentage == None && share_to_issuer_percentage == None {
        return Ok(());
    }

    if share_to_issuer_percentage.unwrap() + share_to_all_members_percentage.unwrap()
        != Uint64::from(100_u64)
    {
        return Err(ContractError::ThreadFeeSharePercentageMustSumTo100 {});
    }

    Ok(())
}
