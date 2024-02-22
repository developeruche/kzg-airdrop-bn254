use ark_bn254::{Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{CurveGroup, VariableBaseMSM};
use ark_ff::BigInt;

pub struct CommitmentScheme {
    poly: Vec<BigInt<4>>, // for quicker msm operation
    srs_g1: Vec<G1Affine>,
    srs_g2: Vec<G2Affine>,
}

impl CommitmentScheme {
    pub fn setup(poly: Vec<BigInt<4>>, srs_g1: Vec<G1Affine>, srs_g2: Vec<G2Affine>) -> Self {
        assert_eq!(
            poly.len(),
            srs_g1.len(),
            "poly length is greater than srs_g1 length"
        );

        CommitmentScheme {
            poly,
            srs_g1,
            srs_g2,
        }
    }

    pub fn commit(&self) -> G1Affine {
        <G1Projective as VariableBaseMSM>::msm_bigint(&self.srs_g1, &self.poly).into_affine()
    }

    // pub fn open(&self, x: BigInt<4>) -> (BigInt<4>, G1Affine) {

    // }
}
