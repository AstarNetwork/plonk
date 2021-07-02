# Plonk For Substrate
[![crates.io badge](https://img.shields.io/crates/v/pallet-plonk.svg)](https://crates.io/crates/pallet-plonk)[![Pallet Test](https://github.com/PlasmNetwork/plonk/actions/workflows/pallet.yml/badge.svg)](https://github.com/PlasmNetwork/plonk/actions/workflows/pallet.yml)  
This library supports for using zkSNARK plonk on substrate and is implemented as an extension of [duck-network plonk](https://github.com/dusk-network/plonk).

## Abstract

We'd like to implement the plonk library as a pallet in order for developers to customize circuits and use the plonk protocol.

The following functions are the libraries we are going to implement as pallet.

- Polynomial commitment
- Circuit builder
- Generate proof and verify keys
- Verify proof

## Details

This is a [Web3 Foundation grant project](https://github.com/w3f/Open-Grants-Program) and our proposal is [here](https://github.com/w3f/Open-Grants-Program/pull/454).  
Our deliverables are following.

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / MIT / Unlicense |
| 0b. | Documentation | We will provide both inline documentation of the code and a basic tutorial that explains how a developer builds a circuit and a user prove their computation through the circuit. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests and audit to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Article/Tutorial | We will publish an article/tutorial/workshop that explains
| 1. | make plonk compatible | The dusk-network plonk is compatible with `no-std` so we are going to modify attributes according to [parity-codec](https://github.com/paritytech/parity-scale-codec) and `Rng` to be compatible with　Substrate environment. This step allows this pallet to work on resource-constrained execution environments like Substrate runtime, attributes should be modified in accordance with SCALE codec and some versions of Rng can’t be compiled to wasm so we need to research and make it stable as necessary. |
| 2. | implement zkSNARK plonk pallet | We will create a set of plonk-based zkSNARK libraries that allow a developer to build their own circuit and a user to prove their computation validity. Verifying proofs are done by on-chain. Creating the proofs are done by off-chain. |  

## Contact
If you have any questions regarding these changes, feel free to contact me at any time.

### Author
Account: [NoCtrlZ](https://github.com/NoCtrlZ)  
Mail: shinsaku.ashizawa@artree.co.jp
