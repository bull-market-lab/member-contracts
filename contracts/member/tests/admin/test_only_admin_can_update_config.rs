use member::ContractError;

use crate::helpers::{assert_err, proper_instantiate, update_config};

#[test]
fn test_only_admin_can_update_config() {
    let (mut app, cw_member_contract_addr, _, _, _, user_1_addr, _) = proper_instantiate();

    assert_err(
        update_config(
            &mut app,
            &cw_member_contract_addr,
            &user_1_addr,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        ContractError::OnlyAdminCanUpdateConfig {},
    );
}
