use cosmwasm_std::{Deps, Uint64};

use crate::{
    state::{ALL_USER_CONFIGS, CONFIG},
    ContractError,
};

pub fn assert_config_fee_share_sum_to_100(deps: Deps) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let share_to_issuer_percentage = config.default_fee_share_config.share_to_issuer_percentage;
    let share_to_all_members_percentage = config
        .default_fee_share_config
        .share_to_all_members_percentage;

    if share_to_issuer_percentage + share_to_all_members_percentage != Uint64::from(100_u64) {
        return Err(ContractError::ThreadFeeSharePercentageMustSumTo100 {});
    }

    Ok(())
}

pub fn assert_user_fee_share_sum_to_100(deps: Deps, user_id: u64) -> Result<(), ContractError> {
    let user_config = ALL_USER_CONFIGS.load(deps.storage, user_id)?;

    if user_config.fee_share_config.is_some() {
        let user_fee_share_config = user_config.fee_share_config.unwrap();
        if user_fee_share_config.share_to_issuer_percentage
            + user_fee_share_config.share_to_all_members_percentage
            != Uint64::from(100_u64)
        {
            return Err(ContractError::ThreadFeeSharePercentageMustSumTo100 {});
        }
    }

    Ok(())
}
