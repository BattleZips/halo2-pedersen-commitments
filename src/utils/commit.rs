use {
    crate::constants::{
        BOARD_COMMITMENT_PERSONALIZATION, BOARD_COMMITMENT_R_BYTES, BOARD_COMMITMENT_V_BYTES,
    },
    halo2_proofs::{arithmetic::CurveExt, pasta::{pallas, group::ff::PrimeField}},
};


pub fn derive_commitment(value: &pallas::Base, rcv: &pallas::Scalar) -> pallas::Point {
    // get curve points used in scalar multiplication
    let hasher = pallas::Point::hash_to_curve(BOARD_COMMITMENT_PERSONALIZATION);
    let V = hasher(&BOARD_COMMITMENT_V_BYTES);
    let R = hasher(&BOARD_COMMITMENT_R_BYTES);
    // convert base field element to scalar
    // https://github.com/zcash/orchard/blob/d05b6cee9df7c4019509e2f54899b5979fb641b5/src/spec.rs#L195
    let value = pallas::Scalar::from_repr(value.to_repr()).unwrap();

    // compute the pedersen commitment for the given value + trapdoor
    V * value + R * rcv
}

#[cfg(test)]
mod test {
    use halo2_proofs::pasta::group::Curve;

    use super::*;

    #[test]
    fn test_pedersen() {
        let value = pallas::Base::from(100);
        let rcv = pallas::Scalar::from(300);
        let commitment = derive_commitment(&value, &rcv).to_affine();
        println!("commitment: {:?}", commitment);
    }
}