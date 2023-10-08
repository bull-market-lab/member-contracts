use cosmwasm_std::{StdError, Uint128, Uint64};
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
    OnlyRegistrationAdminCanRegisterMembershipOnBehalfOfUser {},

    #[error("Only key issuer can update its trading fee config")]
    OnlyMembershipIssuerCanUpdateItsTradingFeeConfig {},

    #[error("Only key issuer can update its thread fee config")]
    OnlyMembershipIssuerCanUpdateItsThreadFeeConfig {},

    #[error("Only key issuer can update its trading fee percentage of key")]
    OnlyMembershipIssuerCanUpdateItsTradingFeePercentageOfMembership {},

    #[error("Only key issuer can update its ask fee percentage of key")]
    OnlyMembershipIssuerCanUpdateItsAskFeePercentageOfMembership {},

    #[error("Only key issuer can update its ask fee to creator percentage of key")]
    OnlyMembershipIssuerCanUpdateItsAskFeeToCreatorPercentageOfMembership {},

    #[error("Only key issuer can update its reply fee percentage of key")]
    OnlyMembershipIssuerCanUpdateItsReplyFeePercentageOfMembership {},

    #[error("User not exist")]
    UserNotExist {},

    #[error("User already registered key")]
    UserAlreadyRegisteredMembership {},

    #[error("User already linked social media")]
    UserAlreadyLinkedSocialMedia {},

    #[error("User has not registered key")]
    UserHasNotRegisteredMembership {},

    #[error("User cannot register key before linking social media")]
    UserCannotRegisterMembershipBeforeLinkingSocialMedia {},

    #[error(
        "Insufficient funds to pay during buy key: needed {needed:?}, available {available:?}"
    )]
    InsufficientFundsToPayDuringBuy { needed: Uint128, available: Uint128 },

    #[error(
        "Insufficient funds to pay during sell key: needed {needed:?}, available {available:?}"
    )]
    InsufficientFundsToPayDuringSell { needed: Uint128, available: Uint128 },

    #[error("Insufficient keys to sell: trying to sell {sell:?}, available {available:?}")]
    InsufficientMembershipsToSell { sell: Uint128, available: Uint128 },

    #[error(
        "Cannot sell last key in supply: trying to sell {sell:?}, total supply {total_supply:?}"
    )]
    CannotSellLastMembership {
        sell: Uint128,
        total_supply: Uint128,
    },

    #[error(
        "All key trading fees must add up to 100 percent: protocol fee {protocol_fee:?}, key issuer fee {key_issuer_fee:?}, key holder fee {key_holder_fee:?}"
    )]
    MembershipTradingFeeDoesNotAddUpTo100Percent {
        protocol_fee: Uint128,
        key_issuer_fee: Uint128,
        key_holder_fee: Uint128,
    },

    #[error(
        "All thread fees must add up to 100 percent: protocol fee {protocol_fee:?}, key issuer fee {key_issuer_fee:?}, key holder fee {key_holder_fee:?}"
    )]
    ThreadFeeDoesNotAddUpTo100Percent {
        protocol_fee: Uint128,
        key_issuer_fee: Uint128,
        key_holder_fee: Uint128,
    },

    #[error("User must hold key to ask")]
    UserMustHoldMembershipToAsk {},

    #[error("User must hold thread creator key to ask in thread")]
    UserMustHoldThreadCreatorMembershipToAskInThread {},

    #[error("User must hold key to reply")]
    UserMustHoldMembershipToReply {},

    #[error("User must have issued key to start new thread")]
    UserMustHaveIssuedMembershipToStartNewThread {},

    #[error("Thread title too long: max {max:?}, actual {actual:?}")]
    ThreadTitleTooLong { max: u64, actual: u64 },

    #[error("Thread description too long: max {max:?}, actual {actual:?}")]
    ThreadDescriptionTooLong { max: u64, actual: u64 },

    #[error("Thread msg content too long: max {max:?}, actual {actual:?}")]
    ThreadMsgContentTooLong { max: u64, actual: u64 },

    #[error(
        "Insufficient funds to pay during ask question: needed {needed:?}, available {available:?}"
    )]
    InsufficientFundsToPayDuringAsk { needed: Uint128, available: Uint128 },

    #[error("Thread already exist")]
    ThreadAlreadyExist {},

    #[error("Thread not exist")]
    ThreadNotExist {},

    #[error("Thread msg already exist")]
    ThreadMsgAlreadyExist {},

    #[error("Thread msg not exist")]
    ThreadMsgNotExist {},

    #[error("Thread msg is not a question")]
    ThreadMsgIsNotQuestion {},

    #[error("Cannot answer others question")]
    CannotAnswerOthersQuestion {},

    #[error("Membership trading fee share percentage must be 100")]
    MembershipTradingFeeSharePercentageMustBe100 {},

    #[error("Thread fee share percentage must be 100")]
    ThreadFeeSharePercentageMustBe100 {},

    #[error("Exceed query limit: given {given:?}, limit {limit:?}")]
    ExceedQueryLimit { given: Uint64, limit: Uint64 },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}
