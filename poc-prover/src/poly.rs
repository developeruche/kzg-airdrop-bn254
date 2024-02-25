use ark_bn254::Fr;
use ark_ff::One;
use ark_poly::{
    univariate::DensePolynomial, DenseUVPolynomial, EvaluationDomain, Evaluations,
    GeneralEvaluationDomain, Radix2EvaluationDomain,
};
use std::ops::{Div, Sub};

use crate::error::{Error, Result};

// given [y1, y2, ..., yn], Returns the f(x) with (n-1) degree passing through points:
// (1, y1), (2, y2), ..., (n, yn)
pub fn poly(points: Vec<Fr>) -> Result<DensePolynomial<Fr>> {
    let domain = GeneralEvaluationDomain::new(points.len())
        .ok_or(Error::PolyError("no poly".to_string()))?;

    let res = Evaluations::from_vec_and_domain(points, domain).interpolate();
    Ok(res)
}

// calculate q(x) = (f(x) - yi) / (x - i)
pub fn vanish(f: &DensePolynomial<Fr>, i: Fr, y_i: Fr) -> DensePolynomial<Fr> {
    let q = f.sub(&DensePolynomial::from_coefficients_vec(vec![y_i]));

    let x = DensePolynomial::from_coefficients_vec(vec![-i, Fr::one()]);
    q.div(&x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_poly::Polynomial;

    #[test]
    fn test_poly() {
        let points = vec![Fr::from(6u64), Fr::from(12u64), Fr::from(20u64)];
        let f = poly(points).unwrap();

        println!("{:?}", f.coeffs());
        assert_eq!(f.evaluate(&Fr::from(1u64)), Fr::from(6u64));
        assert_eq!(f.evaluate(&Fr::from(2u64)), Fr::from(12u64));
        assert_eq!(f.evaluate(&Fr::from(3u64)), Fr::from(20u64));
    }
}
