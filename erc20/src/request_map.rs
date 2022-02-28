//! Implementation of allowances.
use alloc::{string::String};

use casper_contract::{
    contract_api::{storage}, 
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{URef, U256};

use crate::{detail};

#[inline]
pub(crate) fn request_map_uref() -> URef {
    detail::get_uref("request_map")
}

/// Writes an allowance for owner and spender for a specific amount.
pub(crate) fn write_request_map_to(
    request_map_uref: URef,
    unique_id: &String,
    index: U256
) {
    storage::dictionary_put(request_map_uref, unique_id, index)
}

/// Reads an allowance for a owner and spender
pub(crate) fn read_request_map_from(request_map_uref: URef, unique_id: &String) -> U256 {
    storage::dictionary_get(request_map_uref, unique_id)
        .unwrap_or_revert()
        .unwrap_or_default()
}
