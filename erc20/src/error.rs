//! Error handling on the casper platform.
use casper_types::ApiError;

/// Errors which can be returned by the library.
///
/// When an `Error` is returned from a smart contract, it is converted to an [`ApiError::User`].
///
/// Where a smart contract consuming this library needs to define further error variants, it can
/// return those via the [`Error::User`] variant or equivalently via the [`ApiError::User`]
/// variant.
///
/// Such a user error should be in the range `[0..(u16::MAX - 4)]` (i.e. [0, 65532]) to avoid
/// conflicting with the other `Error` variants.
pub enum Error {
    /// ERC20 contract called from within an invalid context.
    InvalidContext,
    /// Spender does not have enough balance.
    InsufficientBalance,
    /// Spender does not have enough allowance approved.
    InsufficientAllowance,
    /// Operation would cause an integer overflow.
    Overflow,
    /// not allow to mint.
    NoAccessRights,
    /// MintTooLow.
    MintTooLow,
    /// AlreadyMint.
    AlreadyMint,
    /// InvalidFee.
    InvalidFee,
    /// request id exist.
    RequestIdExist,
    /// request amount too low.
    RequestAmountTooLow,
    /// User error.
    User(u16),
}

const ERROR_INVALID_CONTEXT: u16 = u16::MAX;
const ERROR_INSUFFICIENT_BALANCE: u16 = u16::MAX - 1;
const ERROR_INSUFFICIENT_ALLOWANCE: u16 = u16::MAX - 2;
const ERROR_OVERFLOW: u16 = u16::MAX - 3;
const NO_ACCESS_RIGHTS: u16 = u16::MAX - 4;
const MINT_TOO_LOW: u16 = u16::MAX - 5;
const ALREADY_MINT: u16 = u16::MAX - 6;
const INVALID_FEE: u16 = u16::MAX - 7;
const REQUEST_ID_EXIST: u16 = u16::MAX - 8;
const REQUEST_AMOUNT_TOO_LOW: u16 = u16::MAX - 9;

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        let user_error = match error {
            Error::InvalidContext => ERROR_INVALID_CONTEXT,
            Error::InsufficientBalance => ERROR_INSUFFICIENT_BALANCE,
            Error::InsufficientAllowance => ERROR_INSUFFICIENT_ALLOWANCE,
            Error::Overflow => ERROR_OVERFLOW,
            Error::NoAccessRights => NO_ACCESS_RIGHTS,
            Error::MintTooLow => MINT_TOO_LOW,
            Error::AlreadyMint => ALREADY_MINT,
            Error::InvalidFee => INVALID_FEE,
            Error::RequestIdExist => REQUEST_ID_EXIST,
            Error::RequestAmountTooLow => REQUEST_AMOUNT_TOO_LOW,
            Error::User(user_error) => user_error,
        };
        ApiError::User(user_error)
    }
}
