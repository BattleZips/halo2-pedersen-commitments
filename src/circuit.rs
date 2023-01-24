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
    pub trapdoor: Value<Fq>,}

// prove knowledge of the message in a given pedersen commitment
impl Circuit<Fp> for PedersenCommitmentCircuit {
    type Config = PedersenCircuitConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        PedersenCommitmentCircuit {
            message: Value::unknown(),
            trapdoor: Value::unknown(),
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

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::utils::{
            commit::derive_commitment,
            get_coordinates
        },
        halo2_proofs::{
            dev::MockProver,
            arithmetic::{Field, CurveAffine},
            pasta::group::Curve
        },
        rand::rngs::OsRng,
    };

    #[test]
    fn ecc_chip() {
        // marshall message into base field element
        let message = Fp::from(88675409);
        // marshall entropy sample for trapdoor into scalar field element
        let trapdoor = Fq::random(&mut OsRng);
        // compute pedersen commitment
        let commitment = derive_commitment(&message, &trapdoor).to_affine();
        let (x, y) = {
            let x = commitment.clone().coordinates().unwrap().x().to_owned();
            let y = commitment.clone().coordinates().unwrap().y().to_owned();
            (x, y)
        };
        // instantiate circuit
        let circuit = PedersenCommitmentCircuit { 
            message: Value::known(message),
            trapdoor: Value::known(trapdoor)
        };
        let prover = MockProver::run(9, &circuit, vec![vec![x, y]]).unwrap();
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
