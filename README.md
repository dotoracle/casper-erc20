# DotOracle's rust smart contracts for Wrapped ERC20 token on Casper

This repo is a fork of [Casper-ERC20](https://github.com/casper-ecosystem/erc20) and is used as code base for wrapped ERC20 token on Casper network

All changes made by DotOracle team can be referred at https://github.com/dotoracle/casper-erc20/tree/minter-role

## Install
Make sure the `wasm32-unknown-unknown` Rust target is installed.
```
make prepare
```

## Build Smart Contracts
To build the example ERC20 contract and supporting test contracts:
```
make build-contracts
```

## Test
```
make test
```

## Javascript client SDK

A javascript client SDK can be used to interact with the ERC20 contract. It is available in it's own [repository](https://github.com/casper-network/casper-contracts-js-clients/tree/master/packages/erc20-client).
