//! Implementation of total supply.

use casper_contract::{contract_api::storage, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{URef};
use alloc::string::{String};

use crate::{constants::MINTER_KEY_NAME, detail};

#[inline]
pub(crate) fn minter_uref() -> URef {
    detail::get_uref(MINTER_KEY_NAME)
}

/// Reads a minter from a specified [`URef`].
pub(crate) fn read_minter_from(uref: URef) -> String {
    storage::read(uref).unwrap_or_revert().unwrap_or_revert()
}

/// Writes a minter to a specific [`URef`].
pub(crate) fn write_minter_to(uref: URef, value: String) {
    storage::write(uref, value);
}
