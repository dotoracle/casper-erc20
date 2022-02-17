#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::string::String;

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_erc20::{
    constants::{
        ADDRESS_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME, DECIMALS_RUNTIME_ARG_NAME,
        MINTER_RUNTIME_ARG_NAME, NAME_RUNTIME_ARG_NAME, OWNER_RUNTIME_ARG_NAME,
        RECIPIENT_RUNTIME_ARG_NAME, SPENDER_RUNTIME_ARG_NAME, SYMBOL_RUNTIME_ARG_NAME,
        TOTAL_SUPPLY_RUNTIME_ARG_NAME,
    },
    Address, ERC20,
};
use casper_types::{CLValue, U256};

#[no_mangle]
pub extern "C" fn name() {
    let name = ERC20::default().name();
    runtime::ret(CLValue::from_t(name).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn symbol() {
    let symbol = ERC20::default().symbol();
    runtime::ret(CLValue::from_t(symbol).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn decimals() {
    let decimals = ERC20::default().decimals();
    runtime::ret(CLValue::from_t(decimals).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let total_supply = ERC20::default().total_supply();
    runtime::ret(CLValue::from_t(total_supply).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn dev() {
    let dev = ERC20::default().dev();
    runtime::ret(CLValue::from_t(dev).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn swap_fee() {
    let swap_fee = ERC20::default().swap_fee();
    runtime::ret(CLValue::from_t(swap_fee).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn origin_chainid() {
    let origin_chainid = ERC20::default().origin_chainid();
    runtime::ret(CLValue::from_t(origin_chainid).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn origin_contract_address() {
    let origin_contract_address = ERC20::default().origin_contract_address();
    runtime::ret(CLValue::from_t(origin_contract_address).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let address: Address = runtime::get_named_arg(ADDRESS_RUNTIME_ARG_NAME);
    let balance = ERC20::default().balance_of(address);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn transfer() {
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default()
        .transfer(recipient, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn approve() {
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default().approve(spender, amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn mint() {
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let swap_fee: U256 = runtime::get_named_arg("swap_fee");
    let mintid: String = runtime::get_named_arg("mintid");
    ERC20::default().mint(recipient, amount, swap_fee, mintid).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn change_minter() {
    let new_minter: Address = runtime::get_named_arg(MINTER_RUNTIME_ARG_NAME);

    ERC20::default().change_minter(new_minter).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn change_dev() {
    let dev: Address = runtime::get_named_arg("dev");

    ERC20::default().change_dev(dev).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn change_swap_fee() {
    let swap_fee: U256 = runtime::get_named_arg("swap_fee");

    ERC20::default().change_swap_fee(swap_fee).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn burn() {
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default().burn(amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn allowance() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let val = ERC20::default().allowance(owner, spender);
    runtime::ret(CLValue::from_t(val).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    ERC20::default()
        .transfer_from(owner, recipient, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn request_bridge_back() {
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let fee: U256 = runtime::get_named_arg("fee");
    let to_chainid: U256 = runtime::get_named_arg("to_chainid");
    let receiver_address: String = runtime::get_named_arg("receiver_address");
    let id: U256 = runtime::get_named_arg("id");

    ERC20::default()
        .request_bridge_back(amount, fee, to_chainid, receiver_address, id)
        .unwrap_or_revert();
}

#[no_mangle]
fn call() {
    let name: String = runtime::get_named_arg(NAME_RUNTIME_ARG_NAME);
    let symbol: String = runtime::get_named_arg(SYMBOL_RUNTIME_ARG_NAME);
    let decimals = runtime::get_named_arg(DECIMALS_RUNTIME_ARG_NAME);
    let total_supply = runtime::get_named_arg(TOTAL_SUPPLY_RUNTIME_ARG_NAME);
    let minter = runtime::get_named_arg(MINTER_RUNTIME_ARG_NAME);
    let swap_fee = runtime::get_named_arg("swap_fee");
    let dev = runtime::get_named_arg("dev");
    let origin_chainid = runtime::get_named_arg("origin_chainid");
    let origin_contract_address = runtime::get_named_arg("origin_contract_address");

    let _token = ERC20::install(name, symbol, decimals, total_supply, minter, swap_fee, dev, origin_chainid, origin_contract_address)
        .unwrap_or_revert();
}
