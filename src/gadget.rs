use {
    crate::constants::fixed_bases::{BoardCommitR, BoardCommitV, BoardFixedBases},
    halo2_gadgets::ecc::{EccInstructions, FixedPoint, FixedPointBaseField, Point, ScalarFixed},
    halo2_proofs::{
        circuit::{AssignedCell, Layouter},
        pasta::pallas,
        plonk::Error,
    },
};

pub fn pedersen_commitment<
    EccChip: EccInstructions<
        pallas::Affine,
        FixedPoints = BoardFixedBases,
        Var = AssignedCell<pallas::Base, pallas::Base>,
    >,
>(
    mut layouter: impl Layouter<pallas::Base>,
    ecc_chip: EccChip,
    // v: FixedPointBaseField<pallas::Affine, EccChip>,
    v: AssignedCell<pallas::Base, pallas::Base>,
    rcv: ScalarFixed<pallas::Affine, EccChip>,
) -> Result<Point<pallas::Affine, EccChip>, Error> {
    // commitment = [v] ValueCommitV
    let commitment = {
        let board_commit_v = FixedPointBaseField::from_inner(ecc_chip.clone(), BoardCommitV);
        board_commit_v.mul(layouter.namespace(|| "[v] BoardCommitV"), v)?
    };

    // blind = [rcv] ValueCommitR
    let (blind, _rcv) = {
        let board_commit_r = BoardCommitR;
        let board_commit_r = FixedPoint::from_inner(ecc_chip, board_commit_r);

        // [rcv] ValueCommitR
        board_commit_r.mul(layouter.namespace(|| "[rcv]BoardCommitR"), rcv)?
    };

    // [v] ValueCommitV + [rcv] ValueCommitR
    commitment.add(layouter.namespace(|| "cv"), &blind)
}
