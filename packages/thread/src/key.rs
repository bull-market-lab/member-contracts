use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct KeyTradingFeeConfig {
    // Protocol fee percentage
    pub protocol_fee_percentage: Uint128,
    // Key issuer fee percentage
    pub key_issuer_fee_percentage: Uint128,
    // Key holder fee percentage
    pub key_holder_fee_percentage: Uint128,
}

#[cw_serde]
pub struct ThreadFeeConfig {
    // Ask fee in key price percentage, e.g. 5 meaning 5% of key price
    pub ask_fee_in_key_price_percentage: Uint128,
    // Protocol fee percentage
    pub protocol_fee_percentage: Uint128,
    // Key issuer fee percentage
    pub key_issuer_fee_percentage: Uint128,
    // Key holder fee percentage
    pub key_holder_fee_percentage: Uint128,
}

#[cw_serde]
pub struct Key {
    // Total number of keys issued
    pub supply: Uint128,
    // Fee config for key trading
    pub key_trading_fee_config: KeyTradingFeeConfig,
    // Fee config for thread
    pub thread_fee_config: ThreadFeeConfig,
}
