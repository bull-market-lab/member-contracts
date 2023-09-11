use cosmwasm_std::{StdError, Uint128};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Only admin can update config")]
    OnlyAdminCanUpdateConfig {},

    #[error("Only key register admin can register key on behalf of user")]
    OnlyKeyRegisterAdminCanRegisterKeyOnBehalfOfUser {},

    #[error("User not exist")]
    UserNotExist {},

    #[error("User already registered key")]
    UserAlreadyRegisteredKey {},

    #[error("User already verified social media")]
    UserAlreadyVerifiedSocialMedia {},

    #[error("Insufficient funds to pay for protocol fee and key price during buy key: required fee {required_fee:?}, available fee {available_fee:?}")]
    InsufficientFundsToPayForProtocolFeeAndPriceDuringBuy {
        required_fee: Uint128,
        available_fee: Uint128,
    },

    #[error("Insufficient funds to pay for protocol fee during sell key: needed {needed:?}, available {available:?}")]
    InsufficientFundsToPayForProtocolFeeDuringSell { needed: Uint128, available: Uint128 },

    #[error("Insufficient keys to sell: trying to sell {sell:?}, available {available:?}")]
    InsufficientKeysToSell { sell: Uint128, available: Uint128 },

    #[error("Cannot sell last key in supply: trying to sell {sell:?}, total supply {total_supply:?}")]
    CannotSellLastKey { sell: Uint128, total_supply: Uint128 },

    #[error("Owner fee too high")]
    KeyIssuerFeeTooHigh {},

    #[error("Protocol fee too high")]
    ProtocolFeeTooHigh {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}
