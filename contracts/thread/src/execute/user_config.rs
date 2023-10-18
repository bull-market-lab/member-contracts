use cosmwasm_std::{Addr, DepsMut, MessageInfo, Response};

use member_pkg::member_contract_querier::query_user_by_id;
use shared_pkg::fee_share_config::FeeShareConfig;
use thread_pkg::{config::FeeConfig, msg::UpdateUserConfigMsg, user_config::UserConfig};

use crate::{
    state::ALL_USER_CONFIGS, util::fee_share::assert_user_fee_share_sum_to_100, ContractError,
};

pub fn update_user_config(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateUserConfigMsg,
    member_contract_addr: Addr,
) -> Result<Response, ContractError> {
    let user_id = data.user_id.u64();
    let user = query_user_by_id(deps.as_ref(), member_contract_addr, user_id);

    if info.sender != user.addr {
        return Err(ContractError::OnlyUserCanUpdateItsOwnConfig {});
    }

    ALL_USER_CONFIGS.update(deps.storage, user_id, |user| match user {
        // User should exist in ALL_USER_CONFIGS as it should be registered
        None => Err(ContractError::UserNotExist {}),
        Some(user) => {
            let user_fee_config = user.fee_config;
            let updated_user = UserConfig {
                fee_config: if data.ask_fee_percentage_of_membership.is_some()
                    || data
                        .ask_fee_to_thread_creator_percentage_of_membership
                        .is_some()
                    || data.reply_fee_percentage_of_membership.is_some()
                    || data
                        .reply_fee_to_thread_creator_percentage_of_membership
                        .is_some()
                {
                    Some(FeeConfig {
                        ask_fee_percentage_of_membership: match data
                            .ask_fee_percentage_of_membership
                        {
                            None => {
                                user_fee_config
                                    .clone()
                                    .unwrap()
                                    .ask_fee_percentage_of_membership
                            }
                            Some(data) => data,
                        },
                        ask_fee_to_thread_creator_percentage_of_membership: match data
                            .ask_fee_to_thread_creator_percentage_of_membership
                        {
                            None => {
                                user_fee_config
                                    .clone()
                                    .unwrap()
                                    .ask_fee_to_thread_creator_percentage_of_membership
                            }
                            Some(data) => data,
                        },
                        reply_fee_percentage_of_membership: match data
                            .reply_fee_percentage_of_membership
                        {
                            None => {
                                user_fee_config
                                    .clone()
                                    .unwrap()
                                    .reply_fee_percentage_of_membership
                            }
                            Some(data) => data,
                        },
                        reply_fee_to_thread_creator_percentage_of_membership: match data
                            .reply_fee_to_thread_creator_percentage_of_membership
                        {
                            None => {
                                user_fee_config
                                    .unwrap()
                                    .reply_fee_to_thread_creator_percentage_of_membership
                            }
                            Some(data) => data,
                        },
                    })
                } else {
                    user_fee_config
                },
                fee_share_config: if data.share_to_all_members_percentage.is_some() {
                    Some(FeeShareConfig {
                        share_to_all_members_percentage: data
                            .share_to_all_members_percentage
                            .unwrap(),
                        share_to_issuer_percentage: data.share_to_issuer_percentage.unwrap(),
                    })
                } else {
                    user.fee_share_config
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
