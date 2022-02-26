//! Implementation of allowances.
use alloc::{string::String};

use casper_contract::{
    contract_api::{storage, runtime}, 
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{URef};

use crate::{detail};

#[inline]
pub(crate) fn mintids_uref() -> URef {
    detail::get_uref("mintids")
}

fn make_dictionary_item_key(mintid: &String) -> String {
    let preimage = mintid.as_bytes();
    let key_bytes = runtime::blake2b(&preimage);
    hex::encode(&key_bytes)
}

/// Writes an allowance for owner and spender for a specific amount.
pub(crate) fn write_mintid_to(
    mintids_uref: URef,
    mintid: &String
) {
    let dictionary_item_key = make_dictionary_item_key(mintid);
    storage::dictionary_put(mintids_uref, &dictionary_item_key, 1)
}

/// Reads an allowance for a owner and spender
pub(crate) fn read_mintid_from(mintids_uref: URef, mintid: &String) -> u64 {
    let dictionary_item_key = make_dictionary_item_key(mintid);
    storage::dictionary_get(mintids_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_default()
}
