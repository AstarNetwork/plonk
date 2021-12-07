use crate::{self as plonk, Config, Transcript};

use dusk_jubjub;
use dusk_plonk::prelude::*;
use frame_support::dispatch::{DispatchErrorWithPostInfo, PostDispatchInfo};
use frame_support::{assert_ok, construct_runtime, parameter_types};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    DispatchError,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

construct_runtime!(
    pub enum TestRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Plonk: plonk::{Module, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for TestRuntime {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Index = u64;
    type Call = Call;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

impl Config for TestRuntime {
    type CustomCircuit = TestCircuit;
    type Event = Event;
}

fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
        .build_storage::<TestRuntime>()
        .unwrap()
        .into()
}

// Implement a circuit that checks:
// 1) a + b = c where C is a PI
// 2) a <= 2^6
// 3) b <= 2^5
// 4) a * b = d where D is a PI
// 5) JubJub::GENERATOR * e(JubJubScalar) = f where F is a Public Input

#[derive(Debug, Default)]
pub struct TestCircuit {
    a: BlsScalar,
    b: BlsScalar,
    c: BlsScalar,
    d: BlsScalar,
    e: JubJubScalar,
    f: JubJubAffine,
}

impl Circuit for TestCircuit {
    const CIRCUIT_ID: [u8; 32] = [0xff; 32];
    fn gadget(&mut self, composer: &mut TurboComposer) -> Result<(), Error> {
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
        let scalar_mul_result =
            composer.component_mul_generator(e, dusk_jubjub::GENERATOR_EXTENDED);
        // Apply the constrain
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

use rand_core::SeedableRng;
use rand_xorshift::XorShiftRng;

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

/// The verify test Ok and Err
#[test]
fn verify() {
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
                f: JubJubAffine::from(dusk_jubjub::GENERATOR_EXTENDED * JubJubScalar::from(2u64)),
            };
            circuit.prove(&pp, &pk, b"Test").unwrap()
        };
        let public_inputs: Vec<PublicInputValue> = vec![
            BlsScalar::from(25u64).into(),
            BlsScalar::from(100u64).into(),
            JubJubAffine::from(dusk_jubjub::GENERATOR_EXTENDED * JubJubScalar::from(2u64)).into(),
        ];
        let fake_public_inputs: Vec<PublicInputValue> = vec![
            BlsScalar::from(24u64).into(),
            BlsScalar::from(100u64).into(),
            JubJubAffine::from(dusk_jubjub::GENERATOR_EXTENDED * JubJubScalar::from(2u64)).into(),
        ];

        assert_ok!(Plonk::verify(
            Origin::signed(1),
            vd.clone(),
            proof.clone(),
            public_inputs,
            Transcript(b"Test")
        ));
        assert_eq!(
            Plonk::verify(
                Origin::signed(1),
                vd,
                proof,
                fake_public_inputs,
                Transcript(b"Test")
            ),
            Err(DispatchErrorWithPostInfo {
                post_info: PostDispatchInfo::from(()),
                error: DispatchError::Other("invalid proof"),
            })
        );
    })
}

/// The plonk integration test only Ok
#[test]
fn plonk() {
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
                f: JubJubAffine::from(dusk_jubjub::GENERATOR_EXTENDED * JubJubScalar::from(2u64)),
            };
            circuit.prove(&pp, &pk, b"Test").unwrap()
        };
        let public_inputs: Vec<PublicInputValue> = vec![
            BlsScalar::from(25u64).into(),
            BlsScalar::from(100u64).into(),
            JubJubAffine::from(dusk_jubjub::GENERATOR_EXTENDED * JubJubScalar::from(2u64)).into(),
        ];

        assert_ok!(Plonk::verify(
            Origin::signed(1),
            vd,
            proof,
            public_inputs,
            Transcript(b"Test")
        ));
    });
}

fn get_rng() -> XorShiftRng {
    XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06,
        0xbc, 0xe5,
    ])
}
