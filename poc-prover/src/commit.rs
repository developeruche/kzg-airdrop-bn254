use ark_bn254::{Bn254, Fr, G1Affine, G1Projective, G2Affine};
use ark_ec::{pairing::Pairing, AffineRepr, CurveGroup, VariableBaseMSM};
use ark_ff::PrimeField;
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial};

use crate::poly::vanish;

pub struct CommitmentScheme {
    poly: DensePolynomial<Fr>, // for quicker msm operation
    srs_g1: Vec<G1Affine>,
    srs_g2: Vec<G2Affine>,
}

impl CommitmentScheme {
    pub fn setup(poly: DensePolynomial<Fr>, srs_g1: Vec<G1Affine>, srs_g2: Vec<G2Affine>) -> Self {
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
        commit(&self.poly, &self.srs_g1)
    }

    pub fn open(&self, x: Fr) -> (Fr, G1Affine) {
        let y = self.poly.evaluate(&x);
        let vanishing_poly = vanish(&self.poly, x, y);
        let point = commit(&vanishing_poly, &self.srs_g1);
        (y, point)
    }

    // e(C- yG1, G2) = e(H, \tau G2 - xG2)
    pub fn verify(&self, commit: G1Affine, y: Fr, proof: G1Affine, x: Fr) -> bool {
        let y_g1: G1Affine = (G1Affine::generator() * y).into();
        let lhs = Bn254::pairing(commit - y_g1, G2Affine::generator());

        let x_g2: G2Affine = (G2Affine::generator() * x).into();
        let rhs = Bn254::pairing(proof, self.srs_g2[1] - x_g2);
        lhs == rhs
    }
}

pub fn commit(poly: &DensePolynomial<Fr>, srs_g1: &[G1Affine]) -> G1Affine {
    let poly = poly
        .coeffs()
        .iter()
        .map(|fr| fr.into_bigint())
        .collect::<Vec<_>>();
    <G1Projective as VariableBaseMSM>::msm_bigint(srs_g1, &poly).into_affine()
}
