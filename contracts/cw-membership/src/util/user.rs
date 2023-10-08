use cosmwasm_std::{Addr, BankMsg, Coin, CosmosMsg, Order, StdResult, Storage, Uint128};
use cw_storage_plus::PrefixBound;

use crate::state::ALL_MEMBERSHIPS_MEMBERS;

pub fn get_cosmos_msgs_to_distribute_fee_to_all_members(
    storage: &mut dyn Storage,
    fee_denom: String,
    total_fee_to_distribute_to_all_key_holders: Uint128,
    key_issuer_addr_ref: &Addr,
    supply: Uint128,
) -> Vec<CosmosMsg> {
    // TODO: P0: revisit, Oh maybe randomly pick one holder to give all the fee, this will solve the out of gas error
    // Maybe pick top 10 holders to give all the fee
    ALL_MEMBERSHIPS_MEMBERS
        .prefix_range(
            storage,
            Some(PrefixBound::inclusive(key_issuer_addr_ref)),
            Some(PrefixBound::inclusive(key_issuer_addr_ref)),
            Order::Ascending,
        )
        .map(|item| {
            item.map(|((_, key_holder), amount)| {
                CosmosMsg::Bank(BankMsg::Send {
                    to_address: key_holder.to_string(),
                    amount: vec![Coin {
                        denom: fee_denom.clone(),
                        amount: total_fee_to_distribute_to_all_key_holders * amount / supply,
                    }],
                })
            })
        })
        .collect::<StdResult<Vec<CosmosMsg>>>()
        .unwrap()
}
