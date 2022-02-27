//! Contains definition of the entry points.
use alloc::{string::String, vec, vec::Vec};

use casper_types::{
    CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter, U256,
};

use crate::{
    address::Address,
    constants::{
        ADDRESS_RUNTIME_ARG_NAME, ALLOWANCE_ENTRY_POINT_NAME, AMOUNT_RUNTIME_ARG_NAME,
        APPROVE_ENTRY_POINT_NAME, BALANCE_OF_ENTRY_POINT_NAME, BURN_ENTRY_POINT_NAME,
        CHANGE_MINTER_ENTRY_POINT_NAME, DECIMALS_ENTRY_POINT_NAME, MINTER_ENTRY_POINT_NAME,
        MINTER_RUNTIME_ARG_NAME, MINT_ENTRY_POINT_NAME, NAME_ENTRY_POINT_NAME,
        OWNER_RUNTIME_ARG_NAME, RECIPIENT_RUNTIME_ARG_NAME, SPENDER_RUNTIME_ARG_NAME,
        SYMBOL_ENTRY_POINT_NAME, TOTAL_SUPPLY_ENTRY_POINT_NAME, TRANSFER_ENTRY_POINT_NAME,
        TRANSFER_FROM_ENTRY_POINT_NAME,
    },
};

/// Returns the `name` entry point.
pub fn name() -> EntryPoint {
    EntryPoint::new(
        String::from(NAME_ENTRY_POINT_NAME),
        Vec::new(),
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `symbol` entry point.
pub fn symbol() -> EntryPoint {
    EntryPoint::new(
        String::from(SYMBOL_ENTRY_POINT_NAME),
        Vec::new(),
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `minter` entry point.
pub fn minter() -> EntryPoint {
    EntryPoint::new(
        String::from(MINTER_ENTRY_POINT_NAME),
        Vec::new(),
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `minter` entry point.
pub fn change_minter() -> EntryPoint {
    EntryPoint::new(
        String::from(CHANGE_MINTER_ENTRY_POINT_NAME),
        vec![Parameter::new(MINTER_RUNTIME_ARG_NAME, Address::cl_type())],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `transfer_from` entry point.
pub fn transfer_from() -> EntryPoint {
    EntryPoint::new(
        String::from(TRANSFER_FROM_ENTRY_POINT_NAME),
        vec![
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(RECIPIENT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `allowance` entry point.
pub fn allowance() -> EntryPoint {
    EntryPoint::new(
        String::from(ALLOWANCE_ENTRY_POINT_NAME),
        vec![
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(SPENDER_RUNTIME_ARG_NAME, Address::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `approve` entry point.
pub fn approve() -> EntryPoint {
    EntryPoint::new(
        String::from(APPROVE_ENTRY_POINT_NAME),
        vec![
            Parameter::new(SPENDER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `transfer` entry point.
pub fn transfer() -> EntryPoint {
    EntryPoint::new(
        String::from(TRANSFER_ENTRY_POINT_NAME),
        vec![
            Parameter::new(RECIPIENT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `mint` entry point.
pub fn mint() -> EntryPoint {
    EntryPoint::new(
        String::from(MINT_ENTRY_POINT_NAME),
        vec![
            Parameter::new(RECIPIENT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
            Parameter::new("swap_fee", U256::cl_type()),
            Parameter::new("mintid", String::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `burn` entry point.
pub fn burn() -> EntryPoint {
    EntryPoint::new(
        String::from(BURN_ENTRY_POINT_NAME),
        vec![Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `request_bridge_back` entry point
pub fn request_bridge_back() -> EntryPoint {
    EntryPoint::new(
        String::from("request_bridge_back"),
        vec![
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
            Parameter::new("fee", U256::cl_type()),
            Parameter::new("to_chainid", U256::cl_type()),
            Parameter::new("receiver_address", String::cl_type()),
            Parameter::new("id", U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `change_swap_fee` entry point
pub fn change_swap_fee() -> EntryPoint {
    EntryPoint::new(
        String::from("change_swap_fee"),
        vec![
            Parameter::new("swap_fee", U256::cl_type())
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `change_dev` entry point
pub fn change_dev() -> EntryPoint {
    EntryPoint::new(
        String::from("change_dev"),
        vec![
            Parameter::new("dev", Address::cl_type())
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `balance_of` entry point.
pub fn balance_of() -> EntryPoint {
    EntryPoint::new(
        String::from(BALANCE_OF_ENTRY_POINT_NAME),
        vec![Parameter::new(ADDRESS_RUNTIME_ARG_NAME, Address::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `total_supply` entry point.
pub fn total_supply() -> EntryPoint {
    EntryPoint::new(
        String::from(TOTAL_SUPPLY_ENTRY_POINT_NAME),
        Vec::new(),
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `decimals` entry point.
pub fn decimals() -> EntryPoint {
    EntryPoint::new(
        String::from(DECIMALS_ENTRY_POINT_NAME),
        Vec::new(),
        u8::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `dev` entry point.
pub fn dev() -> EntryPoint {
    EntryPoint::new(
        String::from("dev"),
        Vec::new(),
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `dev` entry point.
pub fn swap_fee() -> EntryPoint {
    EntryPoint::new(
        String::from("swap_fee"),
        Vec::new(),
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `origin_chainid` entry point.
pub fn origin_chainid() -> EntryPoint {
    EntryPoint::new(
        String::from("origin_chainid"),
        Vec::new(),
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `origin_contract_address` entry point.
pub fn origin_contract_address() -> EntryPoint {
    EntryPoint::new(
        String::from("origin_contract_address"),
        Vec::new(),
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `current_requestid` entry point.
pub fn read_requestid() -> EntryPoint {
    EntryPoint::new(
        String::from("read_requestid"),
        Vec::new(),
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `balance_of` entry point.
pub fn read_mintid() -> EntryPoint {
    EntryPoint::new(
        String::from("read_mintid"),
        vec![Parameter::new("mintid", String::cl_type())],
        u64::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the default set of ERC20 token entry points.
pub fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(name());
    entry_points.add_entry_point(symbol());
    entry_points.add_entry_point(decimals());
    entry_points.add_entry_point(total_supply());
    entry_points.add_entry_point(balance_of());
    entry_points.add_entry_point(transfer());
    entry_points.add_entry_point(approve());
    entry_points.add_entry_point(allowance());
    entry_points.add_entry_point(transfer_from());
    entry_points.add_entry_point(mint());
    entry_points.add_entry_point(burn());
    entry_points.add_entry_point(change_minter());
    entry_points.add_entry_point(change_swap_fee());
    entry_points.add_entry_point(change_dev());
    entry_points.add_entry_point(dev());
    entry_points.add_entry_point(swap_fee());
    entry_points.add_entry_point(origin_chainid());
    entry_points.add_entry_point(origin_contract_address());
    entry_points.add_entry_point(request_bridge_back());
    entry_points.add_entry_point(read_requestid());
    entry_points.add_entry_point(read_mintid());
    entry_points
}
