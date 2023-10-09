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

    #[error("Only admin can update user weight")]
    OnlyAdminCanUpdateUserWeight {},

    #[error("Only admin can update config")]
    OnlyAdminCanUpdateConfig {},

    #[error("Only admin can setup distribution for new membership")]
    OnlyAdminCanSetupDistributionForNewMembership {},

    // ========================== USER ==========================
    #[error("Distribution already setup for membership issuer")]
    DistributionAlreadySetupForMembershipIssuer {},

    #[error("Global indices and effective total weight already setup for membership issuer")]
    GlobalIndicesAndEffectiveTotalWeightAlreadySetupForMembershipIssuer {},

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

    // ========================== OTHERS ==========================

    // #[error(
    //     "All membership trading fees must add up to 100 percent: protocol fee {protocol_fee:?}, membership issuer fee {membership_issuer_fee:?}, membership holder fee {membership_holder_fee:?}"
    // )]
    // MembershipTradingFeeDoesNotAddUpTo100Percent {
    //     protocol_fee: Uint128,
    //     membership_issuer_fee: Uint128,
    //     membership_holder_fee: Uint128,
    // },
    #[error("Membership trading fee share percentage must sum to 100")]
    MembershipTradingFeeSharePercentageMustSumTo100 {},

    #[error("Exceed query limit: given {given:?}, limit {limit:?}")]
    ExceedQueryLimit { given: Uint64, limit: Uint64 },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}
