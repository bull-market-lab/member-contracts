use cosmwasm_std::Uint64;

use member::ContractError;

use crate::helpers::{assert_err, proper_instantiate, update_config};

#[test]
fn test_only_admin_can_update_config() {
    let (mut app, cw_member_contract_addr, admin_addr, _, _, _, _) = proper_instantiate();

    assert_err(
        update_config(
            &mut app,
            &cw_member_contract_addr,
            &admin_addr,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Uint64::from(70_u64)),
            None,
        ),
        ContractError::MembershipTradingFeeSharePercentageMustSumTo100 {},
    );
}
