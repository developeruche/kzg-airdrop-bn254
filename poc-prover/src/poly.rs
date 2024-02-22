use ark_bn254::Fr;
use ark_poly::{
    univariate::DensePolynomial, EvaluationDomain, Evaluations, Radix2EvaluationDomain,
};

use crate::error::{Error, Result};

pub fn poly(points: Vec<Fr>) -> Result<DensePolynomial<Fr>> {
    let domain =
        Radix2EvaluationDomain::new(points.len()).ok_or(Error::PolyError("no poly".to_string()))?;

    let res = Evaluations::from_vec_and_domain(points, domain).interpolate();
    Ok(res)
}
