use cosmwasm_std::{Deps, StdResult};

use crate::state::{ALL_KEYS_HOLDERS, DEFAULT_QUERY_LIMIT, DEFAULT_QUERY_OFFSET};

use friend::{
    key_holder::KeyHolder,
    msg::{KeyHoldersResponse, QueryKeyHoldersMsg},
};

pub fn query_key_holders(deps: Deps, data: QueryKeyHoldersMsg) -> StdResult<KeyHoldersResponse> {
    let offset = data.offset.unwrap_or(DEFAULT_QUERY_OFFSET);
    let limit = data.limit.unwrap_or(DEFAULT_QUERY_LIMIT);
    let key_holders = ALL_KEYS_HOLDERS
        .load(deps.storage, data.key_issuer_addr)?
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect::<Vec<KeyHolder>>();

    Ok(KeyHoldersResponse { key_holders })
}
