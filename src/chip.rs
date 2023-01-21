use {
    crate::{
        constants::{fixed_bases::BoardFixedBases, LOOKUP_SIZE},
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
        circuit::{AssignedCell, Chip, Layouter, Value},
        pasta::{EpAffine, Fp, Fq},
        plonk::{Advice, Column, ConstraintSystem, Error, Fixed, TableColumn},
    },
    std::marker::PhantomData,
};

#[derive(Clone, Debug)]
pub struct PedersenCommitmentConfig<F: FieldExt> {
    pub range_check: LookupRangeCheckConfig<Fp, { LOOKUP_SIZE }>,
    pub ecc: EccConfig<BoardFixedBases>,
    _marker: PhantomData<F>,
}

#[derive(Clone, Debug)]
pub struct PedersenCommitmentChip<F: FieldExt> {
    config: PedersenCommitmentConfig<F>,
}

impl<F: FieldExt> Chip<F> for PedersenCommitmentChip<F> {
    type Config = PedersenCommitmentConfig<F>;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<F: FieldExt> PedersenCommitmentChip<F> {
    pub fn new(config: PedersenCommitmentConfig<F>) -> Self {
        PedersenCommitmentChip {
            config,
        }
    }

    pub fn configure(
        meta: &mut ConstraintSystem<Fp>,
        advice: [Column<Advice>; 10],
        lagrange: [Column<Fixed>; 8],
        lookup: TableColumn,
    ) -> PedersenCommitmentConfig<Fp> {
        // configure range check lookup table chip
        let range_check: LookupRangeCheckConfig<Fp, { LOOKUP_SIZE }> =
            LookupRangeCheckConfig::configure(meta, advice[9], lookup);
        // configure ecc chip
        let ecc = EccChip::<BoardFixedBases>::configure(meta, advice, lagrange, range_check);
        // return configuration
        PedersenCommitmentConfig {
            range_check,
            ecc,
            _marker: PhantomData,
        }
    }

    pub fn synthesize(
        &self,
        mut layouter: impl Layouter<Fp>,
        value: &AssignedCell<Fp, Fp>,
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
            value.clone(),
            trapdoor,
        )?)
    }
}
