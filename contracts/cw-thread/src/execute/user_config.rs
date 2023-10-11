use cosmwasm_std::{Addr, DepsMut, MessageInfo, Response};

use thread::{msg::UpdateUserConfigMsg, user_config::UserConfig};

use crate::{
    state::ALL_USER_CONFIGS,
    util::{fee_share::assert_user_fee_share_sum_to_100, member::query_user_by_id},
    ContractError,
};

pub fn update_user_config(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateUserConfigMsg,
    membership_contract_addr: Addr,
) -> Result<Response, ContractError> {
    let user_id = data.user_id.u64();
    let user = query_user_by_id(deps.as_ref(), membership_contract_addr, user_id);

    if info.sender != user.addr {
        return Err(ContractError::OnlyUserCanUpdateItsOwnConfig {});
    }

    ALL_USER_CONFIGS.update(deps.storage, user_id, |user| match user {
        // User should exist in ALL_USER_CONFIGS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            let updated_user = UserConfig {
                id: data.user_id,
                // ask_fee_percentage_of_membership: Some(data.ask_fee_percentage_of_membership),
                ask_fee_percentage_of_membership: match data.ask_fee_percentage_of_membership {
                    None => user.ask_fee_percentage_of_membership,
                    Some(data) => Some(data),
                },
                ask_fee_to_thread_creator_percentage_of_membership: match data
                    .ask_fee_to_thread_creator_percentage_of_membership
                {
                    None => user.ask_fee_to_thread_creator_percentage_of_membership,
                    Some(data) => Some(data),
                },
                reply_fee_percentage_of_membership: match data.reply_fee_percentage_of_membership {
                    None => user.reply_fee_percentage_of_membership,
                    Some(data) => Some(data),
                },
                reply_fee_to_thread_creator_percentage_of_membership: match data
                    .reply_fee_to_thread_creator_percentage_of_membership
                {
                    None => user.reply_fee_to_thread_creator_percentage_of_membership,
                    Some(data) => Some(data),
                },
                share_to_all_members_percentage: match data.share_to_all_members_percentage {
                    None => user.share_to_all_members_percentage,
                    Some(data) => Some(data),
                },
                share_to_issuer_percentage: match data.share_to_issuer_percentage {
                    None => user.share_to_issuer_percentage,
                    Some(data) => Some(data),
                },
            };
            Ok(updated_user)
        }
    })?;

    assert_user_fee_share_sum_to_100(deps.as_ref(), user_id)?;

    Ok(Response::new()
        .add_attribute("action", "update_user_config")
        .add_attribute("user_id", data.user_id))
}
