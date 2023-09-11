use cosmwasm_std::{Deps, Order, StdResult};
use cw_storage_plus::{Bound, PrefixBound};

use crate::state::{ALL_KEYS_HOLDERS, DEFAULT_QUERY_LIMIT, MAX_QUERY_LIMIT};

use friend::{
    key_holder::KeyHolder,
    msg::{KeyHoldersResponse, QueryKeyHoldersMsg},
};

pub fn query_key_holders(deps: Deps, data: QueryKeyHoldersMsg) -> StdResult<KeyHoldersResponse> {
    let total_count = ALL_KEYS_HOLDERS
        .prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(&data.key_issuer_addr)),
            None,
            Order::Ascending,
        )
        .count();

    let limit = data
        .limit
        .unwrap_or(DEFAULT_QUERY_LIMIT)
        .min(MAX_QUERY_LIMIT) as usize;

    let key_holders = (match data.start_after_user_addr {
        Some(start_after_user_addr) => ALL_KEYS_HOLDERS.range(
            deps.storage,
            Some(Bound::exclusive((
                &data.key_issuer_addr,
                &start_after_user_addr,
            ))),
            None,
            Order::Ascending,
        ),
        None => ALL_KEYS_HOLDERS.prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(&data.key_issuer_addr)),
            None,
            Order::Ascending,
        ),
    })
    .take(limit)
    .map(|item| {
        item.map(|(k, v)| KeyHolder {
            holder_addr: k.1,
            amount: v,
        })
    })
    .collect::<StdResult<Vec<KeyHolder>>>()?;

    Ok(KeyHoldersResponse {
        key_holders,
        total_count,
    })
}
