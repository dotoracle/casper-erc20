//! Implementation of requestids.
use casper_contract::{
    contract_api::{storage},
    unwrap_or_revert::UnwrapOrRevert
};
use casper_types::{URef, U256};
use alloc::string::{ToString};

use crate::{detail};

#[inline]
pub(crate) fn requestids_uref() -> URef {
    detail::get_uref("requestids")
}

/// Writes an requestid 
pub(crate) fn write_requestid_to(
    requestids_uref: URef,
    requestid: U256
) {
    let requestid_str = requestid.to_string();
    storage::dictionary_put(requestids_uref, &requestid_str, 1)
}

/// Reads an allowance for a owner and spender
pub(crate) fn read_requestid_from(requestids_uref: URef, requestid: U256) -> u64 {
    let requestid_str = requestid.to_string();
    storage::dictionary_get(requestids_uref, &requestid_str)
        .unwrap_or_revert()
        .unwrap_or_default()
}
