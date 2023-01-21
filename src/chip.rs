use {
    crate::fixed_bases::{BoardCommitR, BoardCommitV, BoardFixedBases},
    halo2_gadgets::ecc::{EccInstructions, FixedPoint, FixedPointBaseField, Point, ScalarFixed},
    halo2_proofs::{
        circuit::{AssignedCell, Layouter},
        pasta::pallas,
        plonk::Error,
    },
};

pub struct PedersenCommitmentChip

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
        rand::rngs::OsRng
    };

    struct PedersenCircuit {
        test_errors: bool,
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
