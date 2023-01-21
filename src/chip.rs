use halo2_gadgets::ecc::chip::BaseFieldElem;

use {
    crate::{
        constants::{
            fixed_bases::{BoardCommitR, BoardCommitV, BoardFixedBases},
            LOOKUP_SIZE,
        },
        gadget::pedersen_commitment,
    },
    halo2_gadgets::{
        ecc::{
            chip::{EccChip, EccConfig},
            Point, ScalarFixed,
        },
        utilities::lookup_range_check::LookupRangeCheckConfig,
    },
    halo2_proofs::{
        arithmetic::FieldExt,
        circuit::{AssignedCell, Chip, Layouter, NamespacedLayouter, Value},
        pasta::{pallas, EpAffine, Fp, Fq},
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
        lookup: LookupRangeCheckConfig<pallas::Base, { LOOKUP_SIZE }>,
    ) -> PedersenCommitmentConfig {
        let ecc = EccChip::<BoardFixedBases>::configure(meta, advice, lagrange, lookup);
        PedersenCommitmentConfig {
            advice,
            lagrange,
            lookup,
            ecc,
        }
    }

    pub fn synthesize(
        &self,
        mut layouter: impl Layouter<pallas::Base>,
        value: AssignedCell<pallas::Base, pallas::Base>,
        trapdoor: Value<Fq>,
    ) -> Result<Point<EpAffine, EccChip<BoardFixedBases>>, Error> {
        // construct ecc chip
        let ecc_chip = EccChip::construct(self.config.ecc.clone());
        // instantiate commitment trapdoor as a full-width scalar
        let trapdoor = ScalarFixed::new(
            ecc_chip.clone(),
            layouter.namespace(|| "trapdoor"),
            trapdoor,
        )?;
        // synthesize the pedersen commitment computation
        Ok(pedersen_commitment(
            layouter.namespace(|| "pedersen commitment"),
            ecc_chip.clone(),
            value,
            trapdoor,
        )?)
    }
}
