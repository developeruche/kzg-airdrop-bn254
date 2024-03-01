pub mod commit;
pub mod data;
pub mod error;
pub mod poly;

fn main() {}

mod tests {
    use ark_bn254::G2Affine;
    use ark_bn254::{Fr, G1Affine};
    use ark_ec::AffineRepr;
    use ark_ff::Field;
    use ark_std::rand;
    use ark_std::UniformRand;

    fn generate_mock_srs() -> (Vec<G1Affine>, Vec<G2Affine>) {
        let fr = Fr::rand(&mut rand::thread_rng());
        let mut g1 = vec![];

        for i in 0..100u64 {
            g1.push((G1Affine::generator() * fr.pow([i])).into());
        }

        let g2 = vec![G2Affine::generator(), (G2Affine::generator() * fr).into()];
        (g1, g2)
    }

    #[test]
    fn test_generate_mock_srs() {
        use crate::data::PublicParameter;
        use std::fs::File;
        use std::io::{BufWriter, Write};
        let (g1, g2) = generate_mock_srs();
        let para = PublicParameter {
            g1: g1.into_iter().map(|v| format!("{}", v)).collect(),
            g2: g2.into_iter().map(|v| format!("{}", v)).collect(),
        };

        let file = File::create("assets/srs.json").unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &para).unwrap();
        writer.flush().unwrap();
    }
}
