use cosmwasm_std::{BankMsg, Coin, CosmosMsg, Order, StdResult, Storage, Uint128};
use cw_storage_plus::PrefixBound;

use crate::state::{ALL_MEMBERSHIPS_MEMBERS, ALL_USERS};

pub fn get_cosmos_msgs_to_distribute_fee_to_all_members(
    storage: &mut dyn Storage,
    fee_denom: String,
    total_fee_to_distribute_to_all_key_holders: Uint128,
    membership_issuer_user_id: u64,
    supply: Uint128,
) -> Vec<CosmosMsg> {
    // TODO: P0: revisit, Oh maybe randomly pick one holder to give all the fee, this will solve the out of gas error
    // Maybe pick top 10 holders to give all the fee
    ALL_MEMBERSHIPS_MEMBERS
        .prefix_range(
            storage,
            Some(PrefixBound::inclusive(membership_issuer_user_id)),
            Some(PrefixBound::inclusive(membership_issuer_user_id)),
            Order::Ascending,
        )
        .map(|item| {
            item.map(|((_, member_user_id), user_membership_amount)| {
                CosmosMsg::Bank(BankMsg::Send {
                    to_address: ALL_USERS()
                        .idx
                        .id
                        .item(storage, member_user_id)
                        .unwrap()
                        .unwrap()
                        .1
                        .addr
                        .to_string(),
                    amount: vec![Coin {
                        denom: fee_denom.clone(),
                        amount: total_fee_to_distribute_to_all_key_holders * user_membership_amount
                            / supply,
                    }],
                })
            })
        })
        .collect::<StdResult<Vec<CosmosMsg>>>()
        .unwrap()
}
