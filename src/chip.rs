use {
    crate::constants::{
        fixed_bases::{BoardCommitR, BoardCommitV, BoardFixedBases},
        LOOKUP_SIZE,
    },
    halo2_gadgets::{
        ecc::chip::{EccConfig, EccChip},
        utilities::lookup_range_check::LookupRangeCheckConfig,
    },
    halo2_proofs::{
        arithmetic::FieldExt,
        circuit::{AssignedCell, Chip, Layouter},
        pasta::pallas,
        plonk::{Advice, Column, ConstraintSystem, Error, Fixed},
    },
    std::marker::PhantomData,
};

#[derive(Clone, Debug)]
pub struct PedersenCommitmentConfig {
    pub advice: [Column<Advice>; 10],
    pub lagrange: [Column<Fixed>; 8], // fixed lagrange coefficients used in ecc
    pub lookup: LookupRangeCheckConfig<pallas::Base, { LOOKUP_SIZE }>,
    pub ecc: EccConfig<BoardFixedBases>,
}

#[derive(Clone, Debug)]
pub struct PedersenCommitmentChip<F: FieldExt> {
    config: PedersenCommitmentConfig,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> Chip<F> for PedersenCommitmentChip<F> {
    type Config = PedersenCommitmentConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<F: FieldExt> PedersenCommitmentChip<F> {
    pub fn new(config: PedersenCommitmentConfig) -> Self {
        PedersenCommitmentChip {
            config,
            _marker: PhantomData,
        }
    }

    pub fn configure(
        meta: &mut ConstraintSystem<pallas::Base>,
        advice: [Column<Advice>; 10],
        lagrange: [Column<Fixed>; 8],
        lookup: LookupRangeCheckConfig<pallas::Base, { LOOKUP_SIZE }>
    ) -> PedersenCommitmentConfig {
        let ecc = EccChip::configure(meta, advice, lagrange, lookup);
        PedersenCommitmentConfig {
            advice,
            lagrange,
            lookup,
            ecc,
        }
    }

    pub fn synthesize(
        &self,
        mut layouter: impl Layouter<F>,
        value: AssignedCell<F, F>,
        entropy: 
    )
}
#[cfg(test)]
mod tests {
    use halo2_proofs::arithmetic::Field;

    use {
        super::*,
        halo2_gadgets::{
            ecc::chip::{EccChip, EccConfig},
            utilities::lookup_range_check::LookupRangeCheckConfig,
        },
        halo2_proofs::{
            circuit::{Layouter, SimpleFloorPlanner},
            plonk::{Advice, Circuit, Column, ConstraintSystem},
        },
        rand::rngs::OsRng,
    };

    struct PedersenCircuit {
        preimage: pallas::Base,
    }

    #[allow(non_snake_case)]
    impl Circuit<pallas::Base> for PedersenCircuit {
        type Config = EccConfig<BoardFixedBases>;
        type FloorPlanner = SimpleFloorPlanner;

        fn without_witnesses(&self) -> Self {
            PedersenCircuit { test_errors: false }
        }

        fn configure(meta: &mut ConstraintSystem<pallas::Base>) -> Self::Config {
            let advices = [
                meta.advice_column(),
                meta.advice_column(),
                meta.advice_column(),
                meta.advice_column(),
                meta.advice_column(),
                meta.advice_column(),
                meta.advice_column(),
                meta.advice_column(),
                meta.advice_column(),
                meta.advice_column(),
            ];
            let lookup_table = meta.lookup_table_column();
            let lagrange_coeffs = [
                meta.fixed_column(),
                meta.fixed_column(),
                meta.fixed_column(),
                meta.fixed_column(),
                meta.fixed_column(),
                meta.fixed_column(),
                meta.fixed_column(),
                meta.fixed_column(),
            ];
            // Shared fixed column for loading constants
            let constants = meta.fixed_column();
            meta.enable_constant(constants);

            let range_check = LookupRangeCheckConfig::configure(meta, advices[9], lookup_table);
            EccChip::<BoardFixedBases>::configure(meta, advices, lagrange_coeffs, range_check)
        }

        fn synthesize(
            &self,
            config: Self::Config,
            mut layouter: impl Layouter<pallas::Base>,
        ) -> Result<(), Error> {
            let scalar_fixed = pallas::Base::random(OsRng);
        }
    }

    #[test]
    fn ecc_chip() {
        let k = 13;
        let circuit = MyCircuit { test_errors: true };
        let prover = MockProver::run(k, &circuit, vec![]).unwrap();
        assert_eq!(prover.verify(), Ok(()))
    }

    #[cfg(feature = "test-dev-graph")]
    #[test]
    fn print_ecc_chip() {
        use plotters::prelude::*;

        let root = BitMapBackend::new("ecc-chip-layout.png", (1024, 7680)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.titled("Ecc Chip Layout", ("sans-serif", 60)).unwrap();

        let circuit = MyCircuit { test_errors: false };
        halo2_proofs::dev::CircuitLayout::default()
            .render(13, &circuit, &root)
            .unwrap();
    }
}
