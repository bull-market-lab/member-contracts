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

    #[error("Only admin can enable")]
    OnlyAdminCanEnable {},

    #[error("Only admin can disable")]
    OnlyAdminCanDisable {},

    #[error("Only admin can enable open registration")]
    OnlyAdminCanEnableOpenRegistration {},

    #[error("Only admin can disable open registration")]
    OnlyAdminCanDisableOpenRegistration {},

    #[error("Only admin can update config")]
    OnlyAdminCanUpdateConfig {},

    #[error("Only registration admin can link social media on behalf of user")]
    OnlyRegistrationAdminCanLinkSocialMediaOnBehalfOfUser {},

    #[error("Only registration admin can enable membership on behalf of user")]
    OnlyRegistrationAdminCanEnableMembershipOnBehalfOfUser {},

    #[error("Only membership issuer can update its trading fee config")]
    OnlyMembershipIssuerCanUpdateItsTradingFeeConfig {},

    #[error("Only membership issuer can update its trading fee percentage of membership")]
    OnlyMembershipIssuerCanUpdateItsTradingFeePercentageOfMembership {},

    #[error("Only membership issuer can update its ask fee percentage of membership")]
    OnlyMembershipIssuerCanUpdateItsAskFeePercentageOfMembership {},

    #[error("Only membership issuer can update its ask fee to creator percentage of membership")]
    OnlyMembershipIssuerCanUpdateItsAskFeeToCreatorPercentageOfMembership {},

    #[error("Only membership issuer can update its reply fee percentage of membership")]
    OnlyMembershipIssuerCanUpdateItsReplyFeePercentageOfMembership {},

    #[error("User not exist")]
    UserNotExist {},

    #[error("User already registered membership")]
    UserAlreadyRegisteredMembership {},

    #[error("User already linked social media")]
    UserAlreadyLinkedSocialMedia {},

    #[error("User has not registered membership")]
    UserHasNotRegisteredMembership {},

    #[error("User cannot register membership before linking social media")]
    UserCannotRegisterMembershipBeforeLinkingSocialMedia {},

    #[error(
        "Insufficient funds to pay during buy membership: needed {needed:?}, available {available:?}"
    )]
    InsufficientFundsToPayDuringBuy { needed: Uint128, available: Uint128 },

    #[error(
        "Insufficient funds to pay during sell membership: needed {needed:?}, available {available:?}"
    )]
    InsufficientFundsToPayDuringSell { needed: Uint128, available: Uint128 },

    #[error("Insufficient memberships to sell: trying to sell {sell:?}, available {available:?}")]
    InsufficientMembershipsToSell { sell: Uint128, available: Uint128 },

    #[error(
        "Cannot sell last membership in supply: trying to sell {sell:?}, total supply {total_supply:?}"
    )]
    CannotSellLastMembership {
        sell: Uint128,
        total_supply: Uint128,
    },

    // #[error(
    //     "All membership trading fees must add up to 100 percent: protocol fee {protocol_fee:?}, membership issuer fee {membership_issuer_fee:?}, membership holder fee {membership_holder_fee:?}"
    // )]
    // MembershipTradingFeeDoesNotAddUpTo100Percent {
    //     protocol_fee: Uint128,
    //     membership_issuer_fee: Uint128,
    //     membership_holder_fee: Uint128,
    // },

    #[error("User must hold membership to ask")]
    UserMustHoldMembershipToAsk {},

    #[error("User must hold membership to reply")]
    UserMustHoldMembershipToReply {},

    #[error(
        "Insufficient funds to pay during ask question: needed {needed:?}, available {available:?}"
    )]
    InsufficientFundsToPayDuringAsk { needed: Uint128, available: Uint128 },

    #[error("Cannot answer others question")]
    CannotAnswerOthersQuestion {},

    #[error("Membership trading fee share percentage must be 100")]
    MembershipTradingFeeSharePercentageMustBe100 {},

    #[error("Exceed query limit: given {given:?}, limit {limit:?}")]
    ExceedQueryLimit { given: Uint64, limit: Uint64 },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}
