use cosmwasm_std::{StdError, Uint128, Uint64};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    // ========================== ADMIN ==========================
    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Only admin can enable")]
    OnlyAdminCanEnable {},

    #[error("Only admin can disable")]
    OnlyAdminCanDisable {},

    #[error("Only admin can update config")]
    OnlyAdminCanUpdateConfig {},

    // ========================== USER ==========================
    #[error("Only user can update its own config")]
    OnlyUserCanUpdateItsOwnConfig {},

    #[error("Only membership issuer can update its thread fee config")]
    OnlyMembershipIssuerCanUpdateItsThreadFeeConfig {},

    #[error("Only membership issuer can update its ask fee percentage of membership")]
    OnlyMembershipIssuerCanUpdateItsAskFeePercentageOfMembership {},

    #[error("Only membership issuer can update its ask fee to creator percentage of membership")]
    OnlyMembershipIssuerCanUpdateItsAskFeeToCreatorPercentageOfMembership {},

    #[error("Only membership issuer can update its reply fee percentage of membership")]
    OnlyMembershipIssuerCanUpdateItsReplyFeePercentageOfMembership {},

    #[error("User not exist")]
    UserNotExist {},

    // ========================== THREAD ==========================
    #[error("User must hold ask to user membership to ask")]
    UserMustHoldAskToUserMembershipToAsk {},

    #[error("User must hold thread creator membership to ask in its thread")]
    UserMustHoldThreadCreatorMembershipToAskInItsThread {},

    #[error("User must hold thread creator membership to reply")]
    UserMustHoldThreadCreatorMembershipToReply {},

    #[error("User must hold reply to user membership to reply")]
    UserMustHoldThreadReplyToUserMembershipToReply {},

    #[error("User must have issued membership to start new thread")]
    UserMustHaveIssuedMembershipToStartNewThread {},

    #[error("User must have issued membership to ask")]
    UserMustHaveIssuedMembershipToAsk {},

    #[error("User must have issued membership to answer")]
    UserMustHaveIssuedMembershipToAnswer {},

    #[error("User must have issued membership to reply")]
    UserMustHaveIssuedMembershipToReply {},

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

    #[error("Cannot answer non question thread msg")]
    CannotAnswerNonQuestionThreadMsg {},

    #[error("Cannot answer others question")]
    CannotAnswerOthersQuestion {},

    #[error("Thread fee share percentage must sum to 100")]
    ThreadFeeSharePercentageMustSumTo100 {},

    #[error("Exceed query limit: given {given:?}, limit {limit:?}")]
    ExceedQueryLimit { given: Uint64, limit: Uint64 },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}
