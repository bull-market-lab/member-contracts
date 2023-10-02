use cosmwasm_std::{Deps, Order, StdResult};
use cw_storage_plus::{Bound, PrefixBound};

use crate::state::{ALL_KEYS_HOLDERS, DEFAULT_QUERY_LIMIT, MAX_QUERY_LIMIT};

use thread::{
    key_holder::KeyHolder,
    msg::{KeyHoldersResponse, QueryKeyHoldersMsg},
};

pub fn query_key_holders(deps: Deps, data: QueryKeyHoldersMsg) -> StdResult<KeyHoldersResponse> {
    let key_issuer_addr_ref = &deps
        .api
        .addr_validate(data.key_issuer_addr.as_str())
        .unwrap();

    let total_count = ALL_KEYS_HOLDERS
        .prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(key_issuer_addr_ref)),
            Some(PrefixBound::inclusive(key_issuer_addr_ref)),
            Order::Ascending,
        )
        .count();

    let limit = data
        .limit
        .unwrap_or(DEFAULT_QUERY_LIMIT)
        .min(MAX_QUERY_LIMIT) as usize;

    let key_holders: Vec<KeyHolder> = (match data.start_after_user_addr {
        Some(start_after_user_addr) => ALL_KEYS_HOLDERS.range(
            deps.storage,
            Some(Bound::exclusive((
                key_issuer_addr_ref,
                &deps
                    .api
                    .addr_validate(start_after_user_addr.as_str())
                    .unwrap(),
            ))),
            None,
            Order::Ascending,
        ),
        None => ALL_KEYS_HOLDERS.prefix_range(
            deps.storage,
            Some(PrefixBound::inclusive(key_issuer_addr_ref)),
            Some(PrefixBound::inclusive(key_issuer_addr_ref)),
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
        count: key_holders.len(),
        key_holders,
        total_count,
    })
}
