//! Implementation of total supply.

use casper_contract::{contract_api::storage, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{URef, U256};

use crate::{detail};
use alloc::string::{String};

#[inline]
pub(crate) fn get_uref_total_supply() -> URef {
    detail::get_uref("total_supply")
}

#[inline]
pub(crate) fn get_uref_minter() -> URef {
    detail::get_uref("minter")
}

#[inline]
pub(crate) fn get_uref_swap_fee() -> URef {
    detail::get_uref("swap_fee")
}

#[inline]
pub(crate) fn get_uref_dev() -> URef {
    detail::get_uref("dev")
}

/// Reads a total supply from a specified [`URef`].
pub(crate) fn read_total_supply_from(uref: URef) -> U256 {
    storage::read(uref).unwrap_or_revert().unwrap_or_revert()
}

/// Writes a total supply to a specific [`URef`].
pub(crate) fn write_total_supply_to(uref: URef, value: U256) {
    storage::write(uref, value);
}

pub(crate) fn read_minter_from(uref: URef) -> String {
    storage::read(uref).unwrap_or_revert().unwrap_or_revert()
}

/// Writes a minter to a specific [`URef`].
pub(crate) fn write_minter_to(uref: URef, value: String) {
    storage::write(uref, value);
}

/// Reads a total supply from a specified [`URef`].
pub(crate) fn read_swap_fee_from(uref: URef) -> U256 {
    storage::read(uref).unwrap_or_revert().unwrap_or_revert()
}

/// Writes a total supply to a specific [`URef`].
pub(crate) fn write_swap_fee_to(uref: URef, value: U256) {
    storage::write(uref, value);
}

/// Reads a total supply from a specified [`URef`].
pub(crate) fn read_dev_from(uref: URef) -> String {
    storage::read(uref).unwrap_or_revert().unwrap_or_revert()
}

/// Writes a total supply to a specific [`URef`].
pub(crate) fn write_dev_to(uref: URef, value: String) {
    storage::write(uref, value);
}
