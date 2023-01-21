// use std::fs::File;

// use halo2_gadgets::ecc::chip::constants::{compute_lagrange_coeffs, find_zs_and_us};
// use halo2_proofs::{
//     arithmetic::{CurveAffine, CurveExt},
//     pasta::{
//         group::{ff::PrimeField, Curve, Group},
//         pallas,
//     },
// };

// use serde::{Deserialize, Serialize};
// use serde_json::*;

// const NUM_WINDOWS: usize = 85; // 3 bit windows for 255 bit number = 85 windows
// const H: usize = 1 << 3; // idfk

// #[derive(Serialize, Deserialize)]
// struct Data {
//     data: Vec<Vec<Vec<u8>>>,
// }

// fn main() {
//     let point = pallas::Point::hash_to_curve("battlezips:hash2curve")(b"v");
//     let x = point
//         .to_affine()
//         .coordinates()
//         .unwrap()
//         .x()
//         .to_repr()
//         .as_ref();
//     let y = point
//         .to_affine()
//         .coordinates()
//         .unwrap()
//         .y()
//         .to_repr()
//         .as_ref();
//     // println!("x: {:?}",);
//     println!(
//         "y: {:?}",
//         point
//             .to_affine()
//             .coordinates()
//             .unwrap()
//             .y()
//             .to_repr()
//             .as_ref()
//     );
//     // let (z, u): (Vec<u64>, Vec<[pallas::Base; 8]>) = find_zs_and_us(point.to_affine(), NUM_WINDOWS)
//     //     .unwrap()
//     //     .into_iter()
//     //     .unzip();

//     // let u: Vec<Vec<Vec<u8>>> = u
//     //     .iter()
//     //     .map(|x: &[Fp; 8]| {
//     //         x.iter()
//     //             .map(|y: &Fp| y.to_repr().as_ref().to_owned().try_into().unwrap())
//     //             .collect::<Vec<_>>()
//     //     })
//     //     .collect::<Vec<_>>();

//     // let data = serde_json::json!({
//     //     "z": z,
//     //     "u": u
//     // });
//     // to_writer(&File::create("data.json").unwrap(), &data).unwrap();
// }

// /**
//  * Get a generator point with random distribution on pallas curve
//  * @dev point chosen by using hash2curve to pallas with domain separator + personalization
//  *
//  * @param separator - the domain separator specifying the protocol using the commitment
//  * @param personalization - personalization used to differentiate between commitments within protocol
//  * @return - affine (x, y) representation of the poin
//  */
// fn get_generator(separator: &str, personalization: &[u8; 1]) -> pallas::Affine {
//     pallas::Point::hash_to_curve("battlezips:hash2curve")(b"v").to_affine()
// }

// /**
//  * Given an affine point on the pallas curve, return the x and y coordinates
//  * @dev x and y are returned as vectors
//  */
// fn get_generator_coordinates(point: pallas::Affine) -> (Vec<u8>, Vec<u8>) {
//     let x = point.clone().coordinates().unwrap().x().to_repr().as_ref();
//     let y = point.coordinates().unwrap().y().to_repr().as_ref();
//     (x.to_vec(), y.to_vec())
// }
