use cosmwasm_std::{Decimal, Uint128};
use cw_storage_plus::{Item, Map};

use distribution::{config::Config, distribution::Distribution};

pub const DEFAULT_QUERY_LIMIT: u32 = 5;
pub const MAX_QUERY_LIMIT: u32 = 25;

pub const CONFIG: Item<Config> = Item::new("CONFIG");

// /// Total weight of all users eligible for rewards.
// pub const EFFECTIVE_TOTAL_WEIGHT: Item<Uint128> = Item::new("EFFECTIVE_TOTAL_WEIGHT");

/// Tracks global index for rewards.
/// Global index is simply a decimal number representing the amount of currency rewards paid
/// for a unit of user weight, since the beginning of time.
/// Key is membership issuer's user ID., value is (global index, effective total weight).
/// Effective total weight: total weight of all users eligible for rewards.
pub const GLOBAL_INDICES_AND_TOTAL_WEIGHT: Map<u64, (Decimal, Uint128)> =
    Map::new("GLOBAL_INDICES_AND_TOTAL_WEIGHT");

// /// Effective user weights are their weights when taking into account minimum eligible weight
// /// for rewards.
// /// This weight will be the same as user's real weight if they're over the minimum eligible weight,
// /// or 0 if they are under the minimum.
// /// Key is (membership issuer's user ID, member's user ID).
// /// Value is (real weight, effective weight).
// pub const USER_WEIGHTS: Map<(u64, u64), (Uint128, Uint128)> = Map::new("user_weights");

/// Key is (membership issuer's user ID, member's user ID).
/// Value is distribution struct that has user index, pending reward, real weight and effective weight.
pub const USERS_DISTRIBUTIONS: Map<(u64, u64), Distribution> = Map::new("USERS_DISTRIBUTIONS");

// pub struct DistributionIndexes<'a> {
//     pub user: MultiIndex<'a, u64, Distribution, (u64, String)>,
// }

// impl IndexList<Distribution> for DistributionIndexes<'_> {
//     fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Distribution>> + '_> {
//         let v: Vec<&dyn Index<Distribution>> = vec![&self.user];
//         Box::new(v.into_iter())
//     }
// }

// #[allow(non_snake_case)]
// pub fn DISTRIBUTIONS<'a>(
// ) -> IndexedMap<'a, (u64, String), Distribution, DistributionIndexes<'a>> {
//     let indexes = DistributionIndexes {
//         user: MultiIndex::new(
//             |_, distribution| distribution.user.clone(),
//             "distributions",
//             "distributions__user",
//         ),
//     };
//     IndexedMap::new("DISTRIBUTIONS", indexes)
// }

// // convenience trait to unify duplicate code between this and CW20 distributions
// impl From<Distribution> for (Decimal, Uint128) {
//     fn from(item: Distribution) -> Self {
//         (item.user_index, item.pending_rewards)
//     }
// }
