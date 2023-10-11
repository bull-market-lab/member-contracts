use cosmwasm_std::{StdError, Uint64};
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

    #[error("Only membership contract can setup distribution for new membership")]
    OnlyMembershipContractCanSetupDistributionForNewMembership {},

    #[error("Only membership contract can setup distribution for new member")]
    OnlyMembershipContractCanSetupDistributionForNewMember {},

    #[error("Only distribute allowlist addresses can distribute")]
    OnlyDistributeAllowlistAddressesCanDistribute {},

    #[error("Only membership contract can update user pending reward")]
    OnlyMembershipContractCanUpdateUserPendingReward {},

    #[error("Cannot distribute before setup distribution")]
    CannotDistributeBeforeSetupDistribution {},

    #[error("Cannot update pending reward before setup distribution")]
    CannotUpdatePendingRewardBeforeSetupDistribution {},

    #[error("Only admin can add to distribution caller allowlist")]
    OnlyAdminCanAddToDistributionCallerAllowlist {},

    #[error("Only admin can remove from distribution caller allowlist")]
    OnlyAdminCanRemoveFromDistributionCallerAllowlist {},

    #[error("Address not in distribution caller allowlist")]
    AddressNotInDistributionCallerAllowlist {},

    // ========================== USER ==========================
    #[error("Distribution already setup for membership issuer")]
    DistributionAlreadySetupForMembershipIssuer {},

    #[error("Global indices and effective total weight already setup for membership issuer")]
    GlobalIndicesAlreadySetupForMembershipIssuer {},

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

    #[error("Cannot claim reward before setup distribution")]
    CannotClaimRewardBeforeSetupDistribution {},
    // ========================== OTHERS ==========================
    #[error("Exceed query limit: given {given:?}, limit {limit:?}")]
    ExceedQueryLimit { given: Uint64, limit: Uint64 },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}
