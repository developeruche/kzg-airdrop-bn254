use ark_bn254::{Bn254, Fr, G1Affine, G1Projective, G2Affine};
use ark_ec::{pairing::Pairing, AffineRepr, CurveGroup, VariableBaseMSM};
use ark_ff::{Field, PrimeField};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial};

use crate::poly::vanish;

pub struct CommitmentScheme {
    poly: DensePolynomial<Fr>, // for quicker msm operation
    w: Fr,                     // root of unity
    srs_g1: Vec<G1Affine>,
    srs_g2: Vec<G2Affine>,
}

impl CommitmentScheme {
    // poly's length must be 2^k
    pub fn setup(
        poly: DensePolynomial<Fr>,
        w: Fr,
        srs_g1: Vec<G1Affine>,
        srs_g2: Vec<G2Affine>,
    ) -> Self {
        assert_eq!(
            poly.coeffs().len(),
            srs_g1.len(),
            "poly length is greater than srs_g1 length"
        );

        CommitmentScheme {
            poly,
            w,
            srs_g1,
            srs_g2,
        }
    }

    pub fn commit(&self) -> G1Affine {
        commit(&self.poly, &self.srs_g1)
    }

    pub fn open(&self, x_idx: u64) -> (Fr, G1Affine) {
        let x = self.w.pow([x_idx]);
        let y = self.poly.evaluate(&x);
        let vanishing_poly = vanish(&self.poly, x_idx, y, self.w);
        let point = commit(&vanishing_poly, &self.srs_g1);
        (y, point)
    }

    // e(C- yG1, G2) = e(H, \tau G2 - xG2)
    pub fn verify(&self, commit: G1Affine, y: Fr, proof: G1Affine, x_idx: u64) -> bool {
        let x = self.w.pow([x_idx]);
        let y_g1: G1Affine = (G1Affine::generator() * y).into();
        let lhs = Bn254::pairing(commit - y_g1, G2Affine::generator());

        let x_g2: G2Affine = (G2Affine::generator() * x).into();
        let rhs = Bn254::pairing(proof, self.srs_g2[1] - x_g2);
        lhs == rhs
    }
}

pub fn commit(poly: &DensePolynomial<Fr>, srs_g1: &[G1Affine]) -> G1Affine {
    // use std::ops::Mul;

    // poly.coeffs()
    //     .iter()
    //     .zip(srs_g1.iter())
    //     .fold(G1Affine::identity(), |acc, (a_i, g_i)| {
    //         (acc + g_i.mul(a_i)).into()
    //     })

    let poly = poly
        .coeffs()
        .iter()
        .map(|fr| fr.into_bigint())
        .collect::<Vec<_>>();
    <G1Projective as VariableBaseMSM>::msm_bigint(srs_g1, &poly).into_affine()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poly::poly;

    fn new_srs() -> (Vec<G1Affine>, Vec<G2Affine>) {
        let x = vec![1, 3, 9, 27, 81, 243, 729, 2187];
        let x = x.iter().map(|x| Fr::from(*x)).collect::<Vec<Fr>>();
        let g1 = x
            .iter()
            .map(|x| (G1Affine::generator() * x).into())
            .collect::<Vec<G1Affine>>();
        let g2 = vec![G2Affine::generator(), (G2Affine::generator() * x[1]).into()];
        (g1, g2)
    }

    #[test]
    fn test_commitment_scheme() {
        let points = vec![
            Fr::from(12u64),
            Fr::from(123u64),
            Fr::from(1234u64),
            Fr::from(12345u64),
            Fr::from(123456u64),
            Fr::from(1234567u64),
            Fr::from(12345678u64),
            Fr::from(123456789u64),
        ];

        let (f, w) = poly(points).unwrap();
        let cs = CommitmentScheme::setup(f, w, new_srs().0, new_srs().1);

        let commit = cs.commit();

        for i in 0..8 {
            let (y, proof) = cs.open(i);

            assert!(cs.verify(commit, y, proof, i))
        }
    }
}
