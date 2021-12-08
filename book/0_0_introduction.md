# Introduction
We are going to use zk SNARKs on substrate-based blockchain. In this tutorial, we import `plonk-pallet` to substrate runtime, generate the proof and verify.

To use `plonk-pallet` on substrate-based blockchain, we need to do following steps.

1. Define the `plonk-pallet` as depencencies
2. Import the `plonk-pallet` and `plonk-runtime-api` to runtime
3. Create customize `Circuit`
4. Import the `plonk-pallet` and `plonk-pallet-rpc` to rpc node
