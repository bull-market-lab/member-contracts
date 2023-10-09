use cosmwasm_std::Uint128;

/// Calculate user's effective rewards weight, given their actual weight and minimum weight for
/// rewards eligibility
pub fn calculate_effective_weight(weight: Uint128, minimum_eligible_weight: Uint128) -> Uint128 {
    if weight >= minimum_eligible_weight {
        weight
    } else {
        Uint128::zero()
    }
}
