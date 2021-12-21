# Tutorial

We are going to use zk SNARKs on substrate-based blockchain. In this tutorial, we import `plonk-pallet` to substrate runtime, generate the proof and verify.

To use `plonk-pallet` on substrate-based blockchain, we need to do following steps.

1. Define the `plonk-pallet` as depencencies
2. Couple the `plonk-pallet` to your own pallet
3. Define the `plonk-pallet` functions on your pallet
4. Import the coupling pallet to `TestRuntime` and define your `Circuit`
5. Test whether the functions work correctly


## 1.Define the `plonk-pallet` as depencencies
First of all, you need to define the `plonk-pallet` when you start to implement your pallet. Please define as following.

- Cargo.toml
```
[dependencies]
plonk-pallet = { version = '0.2', default-features = false }
rand_core = {version="0.6", default-features = false }
```

The `plonk-pallet` depends on `rand_core` so please import it.

## 2.Couple the `plonk-pallet` to your own pallet
The next, the `plonk-pallet` need to be coupled with your pallet. Please couple the pallet `Config` as following.

- src/lib.rs
```rs
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    pub use plonk_pallet::{ParityRng, Proof, PublicInputValue, Transcript, VerifierData};

    /// Coupling configuration trait with plonk_pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config + plonk_pallet::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }
```
With this step, you can use the `plonk-pallet` in your pallet through `Module`.

## 3.Define the `plonk-pallet` functions on your pallet
The next, let's define the `plonk-pallet` function on your pallet. We are going to define the `trusted_setup` function which generates the public parameters refered as to `srs` and the `verify` function which verified the proof. In this tutorial, we use [sum-storage](https://github.com/JoshOrndorff/recipes/blob/master/pallets/sum-storage/src/lib.rs) pallet as example and add the `verify` function before set `Thing1` storage value on `set_thing_1`. If the `verify` is success, the `set_thing_1` can set `Thing1` value.

- src/lib.rs
```rs
    // The module's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Coupled trusted setup
        #[pallet::weight(10_000)]
        pub fn trusted_setup(
            origin: OriginFor<T>,
            val: u32,
            rng: ParityRng,
        ) -> DispatchResultWithPostInfo {
            plonk_pallet::Pallet::<T>::trusted_setup(origin, val, rng)?;
            Ok(().into())
        }

    /// Sets the first simple storage value
    #[pallet::weight(10_000)]
    pub fn set_thing_1(
        origin: OriginFor<T>,
        val: u32,
        vd: VerifierData,
        proof: Proof,
        public_inputs: Vec<PublicInputValue>,
        transcript_init: Transcript,
    ) -> DispatchResultWithPostInfo {
        // Define the proof varification
        plonk_pallet::Pallet::<T>::verify(origin, vd, proof, public_inputs, transcript_init)?;

        Thing1::<T>::put(val);

        Self::deposit_event(Event::ValueSet(1, val));
        Ok(().into())
    }
```
With this step, we can check whether the proof is valid before setting the `Thing1` value and only if the proof is valid, the value is set.

## 4.Import the coupling pallet to `TestRuntime` and define your `Circuit`
We already imported the `plonk-pallet` functions so we are going to import it to `TestRumtime` and define your customized `Circuit`.

In order to use `plonk-pallet` in `TestRuntime`, we need to import `plonk-pallet` crate and define the pallet config to `construct_runtime` as following.

- src/tests.rs
```
use crate::{self as sum_storage, Config};

use frame_support::dispatch::{DispatchError, DispatchErrorWithPostInfo, PostDispatchInfo};
use frame_support::{assert_ok, construct_runtime, parameter_types};

// Import `plonk_pallet` and dependency
pub use plonk_pallet::*;
use rand_core::SeedableRng;

--- snip ---

construct_runtime!(
    pub enum TestRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        // Define the `plonk_pallet` in `contruct_runtime`
        Plonk: plonk_pallet::{Module, Call, Storage, Event<T>},
        {YourPallet}: {your_pallet}::{Module, Call, Storage, Event<T>},
    }
);
```

As the final step of runtime configuration, we define the zk-SNARKs circuit and extend the `TestRuntime` config with it. You can replace `TestCircuit` with your own circuit.

```rs
// Implement a circuit that checks:
// 1) a + b = c where C is a PI
// 2) a <= 2^6
// 3) b <= 2^5
// 4) a * b = d where D is a PI
// 5) JubJub::GENERATOR * e(JubJubScalar) = f where F is a Public Input

#[derive(Debug, Default)]
pub struct TestCircuit {
    pub a: BlsScalar,
    pub b: BlsScalar,
    pub c: BlsScalar,
    pub d: BlsScalar,
    pub e: JubJubScalar,
    pub f: JubJubAffine,
}

impl Circuit for TestCircuit {
    const CIRCUIT_ID: [u8; 32] = [0xff; 32];
    fn gadget(&mut self, composer: &mut TurboComposer) -> Result<(), PlonkError> {
        let a = composer.append_witness(self.a);
        let b = composer.append_witness(self.b);

        // Make first constraint a + b = c
        let constraint = Constraint::new().left(1).right(1).public(-self.c).a(a).b(b);

        composer.append_gate(constraint);

        // Check that a and b are in range
        composer.component_range(a, 1 << 6);
        composer.component_range(b, 1 << 5);

        // Make second constraint a * b = d
        let constraint = Constraint::new()
            .mult(1)
            .output(1)
            .public(-self.d)
            .a(a)
            .b(b);

        composer.append_gate(constraint);

        let e = composer.append_witness(self.e);
        let scalar_mul_result = composer.component_mul_generator(e, GENERATOR_EXTENDED);
        composer.assert_equal_public_point(scalar_mul_result, self.f);
        Ok(())
    }

    fn public_inputs(&self) -> Vec<PublicInputValue> {
        vec![self.c.into(), self.d.into(), self.f.into()]
    }

    fn padded_gates(&self) -> usize {
        1 << 11
    }
}

impl plonk_pallet::Config for TestRuntime {
    type CustomCircuit = TestCircuit;
    type Event = Event;
}
```
With this step, we finish to setup the plonk runtime environment.

## 5.Test whether the functions work correctly
The plonk functions is available on your pallet so we are going to test them as following tests.
```rs

/// The trusted setup test Ok and Err
#[test]
fn trusted_setup() {
    new_test_ext().execute_with(|| {
        let rng = get_rng();
        assert_ok!(Plonk::trusted_setup(Origin::signed(1), 12, rng));

        let rng = get_rng();
        assert_eq!(
            Plonk::trusted_setup(Origin::signed(1), 12, rng),
            Err(DispatchErrorWithPostInfo {
                post_info: PostDispatchInfo::from(()),
                error: DispatchError::Other("already setup"),
            })
        );
    })
}

/// The set `Thing1` storage with valid proof
#[test]
fn sums_thing_one_with_valid_proof() {
    new_test_ext().execute_with(|| {
        let rng = get_rng();
        assert_ok!(Plonk::trusted_setup(Origin::signed(1), 12, rng));

        let pp = Plonk::public_parameter().unwrap();
        let mut circuit = TestCircuit::default();
        let (pk, vd) = circuit.compile(&pp).unwrap();
        let proof = {
            let mut circuit = TestCircuit {
                a: BlsScalar::from(20u64),
                b: BlsScalar::from(5u64),
                c: BlsScalar::from(25u64),
                d: BlsScalar::from(100u64),
                e: JubJubScalar::from(2u64),
                f: JubJubAffine::from(GENERATOR_EXTENDED * JubJubScalar::from(2u64)),
            };
            circuit.prove(&pp, &pk, b"Test").unwrap()
        };
        let public_inputs: Vec<PublicInputValue> = vec![
            BlsScalar::from(25u64).into(),
            BlsScalar::from(100u64).into(),
            JubJubAffine::from(GENERATOR_EXTENDED * JubJubScalar::from(2u64)).into(),
        ];

        assert_ok!(SumStorage::set_thing_1(Origin::signed(1), 42, vd, proof, public_inputs, Transcript(b"Test")));
        assert_eq!(SumStorage::get_sum(), 42);
    });
}

/// The set `Thing1` storage with invalid proof
#[test]
fn sums_thing_one_with_invalid_proof() {
    new_test_ext().execute_with(|| {
        let rng = get_rng();
        assert_ok!(Plonk::trusted_setup(Origin::signed(1), 12, rng));

        let pp = Plonk::public_parameter().unwrap();
        let mut circuit = TestCircuit::default();
        let (pk, vd) = circuit.compile(&pp).unwrap();
        let proof = {
            let mut circuit = TestCircuit {
                a: BlsScalar::from(20u64),
                b: BlsScalar::from(5u64),
                c: BlsScalar::from(25u64),
                d: BlsScalar::from(100u64),
                e: JubJubScalar::from(2u64),
                f: JubJubAffine::from(GENERATOR_EXTENDED * JubJubScalar::from(2u64)),
            };
            circuit.prove(&pp, &pk, b"Test").unwrap()
        };
        let public_inputs: Vec<PublicInputValue> = vec![
            // Change the value
            BlsScalar::from(24u64).into(),
            BlsScalar::from(100u64).into(),
            JubJubAffine::from(GENERATOR_EXTENDED * JubJubScalar::from(2u64)).into(),
        ];

        assert!(SumStorage::set_thing_1(Origin::signed(1), 42, vd, proof, public_inputs, Transcript(b"Test")).is_err());
        assert_eq!(SumStorage::get_sum(), 0);
    });
}
```
With above tests, we can confirm that your pallet is coupling with `plonk-pallet` and these functions work correctly. You can check the `plonk-pallet` specification [here](https://docs.rs/plonk-pallet/latest/plonk_pallet/). Happy hacking!
