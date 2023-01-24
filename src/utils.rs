use halo2_proofs::{
    arithmetic::CurveAffine,
    pasta::{group::ff::PrimeField, EpAffine, Fp},
};

pub mod commit;
pub mod fixed_bases;

/**
 * Given an affine point on the base field, parse x and y coordinates
 *
 * @return (x, y) - 256-bit x and y coordinates for pedersen commitment
 */
pub fn get_coordinates(point: EpAffine) -> (Fp, Fp) {
    let x = point.clone().coordinates().unwrap().x().to_owned();
    let y = point.clone().coordinates().unwrap().y().to_owned();
    (x, y)
}
