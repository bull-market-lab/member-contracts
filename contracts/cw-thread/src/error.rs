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

    #[error("Only registration admin can link social media on behalf of user")]
    OnlyRegistrationAdminCanLinkSocialMediaOnBehalfOfUser {},

    #[error("Only registration admin can register key on behalf of user")]
    OnlyRegistrationAdminCanRegisterKeyOnBehalfOfUser {},

    #[error("Only key issuer can update its trading fee config")]
    OnlyKeyIssuerCanUpdateItsTradingFeeConfig {},

    #[error("Only key issuer can update its QA fee config")]
    OnlyKeyIssuerCanUpdateItsQAFeeConfig {},

    #[error("User not exist")]
    UserNotExist {},

    #[error("User already registered key")]
    UserAlreadyRegisteredKey {},

    #[error("User already linked social media")]
    UserAlreadyLinkedSocialMedia {},

    #[error("User has not registered key")]
    UserHasNotRegisteredKey {},

    #[error("User cannot register key before linking social media")]
    UserCannotRegisterKeyBeforeLinkingSocialMedia {},

    #[error(
        "Insufficient funds to pay during buy key: needed {needed:?}, available {available:?}"
    )]
    InsufficientFundsToPayDuringBuy { needed: Uint128, available: Uint128 },

    #[error(
        "Insufficient funds to pay during sell key: needed {needed:?}, available {available:?}"
    )]
    InsufficientFundsToPayDuringSell { needed: Uint128, available: Uint128 },

    #[error("Insufficient keys to sell: trying to sell {sell:?}, available {available:?}")]
    InsufficientKeysToSell { sell: Uint128, available: Uint128 },

    #[error(
        "Cannot sell last key in supply: trying to sell {sell:?}, total supply {total_supply:?}"
    )]
    CannotSellLastKey {
        sell: Uint128,
        total_supply: Uint128,
    },

    #[error(
        "All key trading fees must add up to 100 percent: protocol fee {protocol_fee:?}, key issuer fee {key_issuer_fee:?}, key holder fee {key_holder_fee:?}"
    )]
    KeyTradingFeeDoesNotAddUpTo100Percent {
        protocol_fee: Uint128,
        key_issuer_fee: Uint128,
        key_holder_fee: Uint128,
    },

    #[error(
        "All QA fees must add up to 100 percent: protocol fee {protocol_fee:?}, key issuer fee {key_issuer_fee:?}, key holder fee {key_holder_fee:?}"
    )]
    QAFeeDoesNotAddUpTo100Percent {
        protocol_fee: Uint128,
        key_issuer_fee: Uint128,
        key_holder_fee: Uint128,
    },

    #[error("User must hold key to ask")]
    UserMustHoldKeyToAsk {},

    #[error("QA thread title too long: max {max:?}, actual {actual:?}")]
    QAThreadTitleTooLong { max: u64, actual: u64 },

    #[error("QA thread msg content too long: max {max:?}, actual {actual:?}")]
    QAThreadMsgContentTooLong { max: u64, actual: u64 },

    #[error(
        "Insufficient funds to pay during ask question: needed {needed:?}, available {available:?}"
    )]
    InsufficientFundsToPayDuringAsk { needed: Uint128, available: Uint128 },

    #[error("QA thread already exist")]
    QAThreadAlreadyExist {},

    #[error("QA thread not exist")]
    QAThreadNotExist {},

    #[error("QA thread msg already exist")]
    QAThreadMsgAlreadyExist {},

    #[error("QA thread msg not exist")]
    QAThreadMsgNotExist {},

    #[error("Only key issuer can answer question")]
    OnlyKeyIssuerCanAnswer {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}
