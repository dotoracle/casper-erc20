//! Implementation of allowances.
use alloc::{string::String};

use casper_contract::{
    contract_api::{storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{URef};

use crate::{detail};

#[inline]
pub(crate) fn mintids_uref() -> URef {
    detail::get_uref("mintids")
}

/// Writes an allowance for owner and spender for a specific amount.
pub(crate) fn write_mintid_to(
    mintids_uref: URef,
    mintid: &String
) {
    storage::dictionary_put(mintids_uref, mintid, 1)
}

/// Reads an allowance for a owner and spender
pub(crate) fn read_mintid_from(mintids_uref: URef, mintid: &String) -> u64 {
    storage::dictionary_get(mintids_uref, mintid)
        .unwrap_or_revert()
        .unwrap_or_default()
}
