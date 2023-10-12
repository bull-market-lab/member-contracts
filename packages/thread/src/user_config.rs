use cosmwasm_schema::cw_serde;
use shared::fee_share_config::FeeShareConfig;

use crate::config::FeeConfig;

// User doesn't need to sign up to use the thread, as long as user sign up with the membership contract
// It has access to the thread contract.
// User struct here is only storing thread specific user config, it can be seen as an extension of the user struct in membership contract
#[cw_serde]
pub struct UserConfig {
    pub fee_config: Option<FeeConfig>,
    pub fee_share_config: Option<FeeShareConfig>,
}
