use cosmwasm_std::{Deps, Uint64};
use member::config::FeeShareConfig;

use crate::{state::ALL_USERS, ContractError};

pub fn assert_config_fee_share_sum_to_100(
    default_fee_share_config: FeeShareConfig,
) -> Result<(), ContractError> {
    if default_fee_share_config.share_to_issuer_percentage
        + default_fee_share_config.share_to_all_members_percentage
        != Uint64::from(100_u64)
    {
        return Err(ContractError::MembershipTradingFeeSharePercentageMustSumTo100 {});
    }

    Ok(())
}

pub fn assert_user_fee_share_sum_to_100(deps: Deps, user_id: u64) -> Result<(), ContractError> {
    let user = ALL_USERS().idx.id.item(deps.storage, user_id)?.unwrap().1;

    if user.fee_share_config.is_some() {
        let user_fee_share_config = user.fee_share_config.unwrap();
        if user_fee_share_config.share_to_issuer_percentage
            + user_fee_share_config.share_to_all_members_percentage
            != Uint64::from(100_u64)
        {
            return Err(ContractError::MembershipTradingFeeSharePercentageMustSumTo100 {});
        }
    }

    Ok(())
}
