# Plonk For Substrate
 [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE) [![crates.io badge](https://img.shields.io/crates/v/plonk-pallet.svg)](https://crates.io/crates/plonk-pallet) [![Pallet Test](https://github.com/PlasmNetwork/plonk/actions/workflows/pallet.yml/badge.svg)](https://github.com/PlasmNetwork/plonk/actions/workflows/pallet.yml)  
This is `plonk` pallet implemented as an extension of [duck-network plonk](https://github.com/dusk-network/plonk).

## Abstract

We'd like to implement the plonk library as a pallet in order for developers to customize circuits and use the plonk protocol.

The following functions are the libraries we are going to implement as pallet and client.

- Custom circuit builder
- Trusted setup
- Generate proof
- Verify proof

## Reference

The tutorial importing `plonk-pallet` to runtime and node.
- [Tutorial](https://astarnetwork.github.io/plonk/)  

The `pallet` dependencies rustdoc.
- [`plonk-pallet`](https://docs.rs/plonk-pallet/latest/plonk_pallet/)  
- [`plonk-pallet-rpc`](https://docs.rs/plonk-pallet-rpc/latest/plonk_pallet_rpc/)  
- [`plonk-runtime-api`](https://docs.rs/plonk-runtime-api/latest/plonk_runtime_api/)
