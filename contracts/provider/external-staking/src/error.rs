use cosmwasm_std::{ConversionOverflowError, StdError, Uint128};
use cw_utils::PaymentError;
use mesh_apis::ibc::VersionError;
use mesh_sync::{RangeError, Tx};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("{0}")]
    IbcVersion(#[from] VersionError),

    #[error("{0}")]
    Conversion(#[from] ConversionOverflowError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Unauthorized. Received connection id: {0}, counterparty port id: {1}. Expected connection id: {2}, counterparty port id: {3}.")]
    IbcUnauthorized(String, String, String, String),

    #[error("Invalid denom, {0} expected")]
    InvalidDenom(String),

    #[error("You cannot specify a slash ratio over 1.0 (100%)")]
    InvalidSlashRatio,

    #[error("Not enough tokens staked, up to {0} can be unbond")]
    NotEnoughStake(Uint128),

    #[error("Not enough tokens released, up to {0} can be claimed")]
    NotEnoughRelease(Uint128),

    #[error("Validator for user mismatch, {0} expected")]
    InvalidValidator(String),

    #[error("Cannot stake to {0}, not listed as an active validator on consumer")]
    ValidatorNotActive(String),

    #[error("Contract already has an open IBC channel")]
    IbcChannelAlreadyOpen,

    #[error("You must start the channel handshake on the other side, it doesn't support OpenInit")]
    IbcOpenInitDisallowed,

    #[error("Invalid authorized endpoint: {0}")]
    InvalidEndpoint(String),

    #[error("The tx {0} exists but is of the wrong type: {1}")]
    WrongTypeTx(u64, Tx),

    #[error("No staking rewards to be withdrawn")]
    NoRewards,

    #[error("Validator '{0}' already tombstoned / not found at height {1}")]
    AlreadyTombstoned(String, u64),

    #[error("{0}")]
    Range(#[from] RangeError),

    #[error("User {0} has not enough delegated funds: {1}")]
    InsufficientDelegations(String, Uint128),
}
