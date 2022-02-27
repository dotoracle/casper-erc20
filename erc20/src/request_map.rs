//! Implementation of allowances.
use alloc::{string::String};

use casper_contract::{
    contract_api::{storage, runtime}, 
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{URef, U256};

use crate::{detail};

#[inline]
pub(crate) fn request_map_uref() -> URef {
    detail::get_uref("request_map")
}

fn make_dictionary_item_key(unique_id: &String) -> String {
    let preimage = unique_id.as_bytes();
    let key_bytes = runtime::blake2b(&preimage);
    hex::encode(&key_bytes)
}

/// Writes an allowance for owner and spender for a specific amount.
pub(crate) fn write_request_map_to(
    request_map_uref: URef,
    unique_id: &String,
    index: U256
) {
    let dictionary_item_key = make_dictionary_item_key(unique_id);
    storage::dictionary_put(request_map_uref, &dictionary_item_key, index)
}

/// Reads an allowance for a owner and spender
pub(crate) fn read_request_map_from(request_map_uref: URef, unique_id: &String) -> U256 {
    let dictionary_item_key = make_dictionary_item_key(unique_id);
    storage::dictionary_get(request_map_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_default()
}
