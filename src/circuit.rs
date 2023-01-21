use {
    crate::{
        chip::{PedersenCommitmentChip, PedersenCommitmentConfig},
        constants::fixed_bases::BoardFixedBases,
    },
    halo2_gadgets::ecc::chip::EccConfig,
    halo2_proofs::{
        arithmetic::FieldExt,
        circuit::{Layouter, SimpleFloorPlanner, Value},
        pasta::{pallas, Fp, Fq},
        plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance},
    },
    std::marker::PhantomData,
};

#[derive(Clone, Debug)]

pub struct PedersenCircuitConfig {
    pub instance: Column<Instance>,
    pub pedersen: PedersenCommitmentConfig<Fp>,
}

#[derive(Clone, Debug)]

pub struct PedersenCommitmentCircuit {
    pub message: Value<Fp>,
    pub trapdoor: Value<Fq>,
    pub config: PedersenCircuitConfig,
}

// prove knowledge of the message in a given pedersen commitment
impl Circuit<Fp> for PedersenCommitmentCircuit {
    type Config = PedersenCircuitConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        PedersenCommitmentCircuit {
            message: Value::unknown(),
            trapdoor: Value::unknown(),
            config: self.config.clone(),
        }
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let advice = [
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

        // Instance column to export pedersen commitment publicly
        let instance = meta.instance_column();
        meta.enable_equality(instance);

        let pedersen =
            PedersenCommitmentChip::<Fp>::configure(meta, advice, lagrange_coeffs, lookup_table);

        PedersenCircuitConfig { instance, pedersen }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        // retrieve advice column from inside ecc chip to witness message
        // column management is intended to be handled at higher level circuit & wont need such references in prod use
        let advice = config.pedersen.ecc.advices[0];
        // witness private inputs
        let message = &layouter.assign_region(
            || "witness message",
            |mut region| {
                let message =
                    region.assign_advice(|| "witness message", advice, 0, || self.message)?;
                Ok(message)
            },
        )?;
        // synthesize pedersen commitment
        let chip = PedersenCommitmentChip::<Fp>::new(config.pedersen.clone());
        let commitment =
            chip.synthesize(layouter.namespace(|| "pedersen"), message, self.trapdoor)?;
        // export constrained pedersen commitment to instance column
        let x = commitment.clone().inner().x().cell();
        let y = commitment.clone().inner().y().cell();
        layouter.constrain_instance(x, config.instance, 0)?;
        layouter.constrain_instance(y, config.instance, 1)?;
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use halo2_proofs::arithmetic::Field;

//     use {
//         super::*,
//         halo2_gadgets::{
//             ecc::chip::{EccChip, EccConfig},
//             utilities::lookup_range_check::LookupRangeCheckConfig,
//         },
//         halo2_proofs::{
//             circuit::{Layouter, SimpleFloorPlanner},
//             plonk::{Advice, Circuit, Column, ConstraintSystem},
//         },
//         rand::rngs::OsRng,
//     };

//     #[test]
//     fn ecc_chip() {
//         let k = 13;
//         let circuit = MyCircuit { test_errors: true };
//         let prover = MockProver::run(k, &circuit, vec![]).unwrap();
//         assert_eq!(prover.verify(), Ok(()))
//     }

//     #[cfg(feature = "test-dev-graph")]
//     #[test]
//     fn print_ecc_chip() {
//         use plotters::prelude::*;

//         let root = BitMapBackend::new("ecc-chip-layout.png", (1024, 7680)).into_drawing_area();
//         root.fill(&WHITE).unwrap();
//         let root = root.titled("Ecc Chip Layout", ("sans-serif", 60)).unwrap();

//         let circuit = MyCircuit { test_errors: false };
//         halo2_proofs::dev::CircuitLayout::default()
//             .render(13, &circuit, &root)
//             .unwrap();
//     }
// }
