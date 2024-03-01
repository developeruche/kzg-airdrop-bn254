use crate::error::{Error, Result};
use ark_bn254::{Fr, G1Affine, G2Affine};
use ark_ff::BigInt;
use csv::Reader;
use ethers::{
    abi::{encode, Token},
    types::{Address, U256},
    utils::keccak256,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SRSRaw {
    transcripts: Vec<Transcripts>,
    participant_ids: Vec<String>,
    participant_ecdsa_signatures: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Transcripts {
    num_g1_powers: u64,
    num_g2_powers: u64,
    powers_of_tau: PowersOfTau,
    witness: Witness,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct PowersOfTau {
    g1_powers: Vec<String>,
    g2_powers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Witness {
    running_products: Vec<String>,
    pot_pubkeys: Vec<String>,
    bls_signatures: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicParameter {
    pub g1: Vec<String>,
    pub g2: Vec<String>,
}

pub fn read_srs(path: &str) -> Result<PublicParameter> {
    let json_string = fs::read_to_string(path).expect("Unable to read file");

    let srs_json: PublicParameter =
        serde_json::from_str(&json_string).expect("JSON does not have correct format.");

    let mut g1 = vec![];
    
    for g1 in &srs_json.g1 {
        g1.push(G1Affine::from_str(g1).map_err(|_| Error::Internal("Invalid G1".to_string()))?);
    }
    let g2 = vec![G2Affine::try_from(value)]
    let g1 = s;
}

pub fn read_user_data(path: &str) -> Result<Vec<Fr>> {
    let mut data = Reader::from_path(path)?;
    data.records()
        .map(|record| {
            let record = record?;
            let addr = record
                .get(0)
                .ok_or(Error::Internal("No value".to_string()))?
                .trim();

            let amount = record
                .get(1)
                .ok_or(Error::Internal("No value".to_string()))?
                .trim();

            let encoded_data = encode(&[
                Token::Address(Address::from_str(addr).unwrap()),
                Token::Uint(U256::from_dec_str(amount).unwrap()),
            ]);

            let mut hash = keccak256(encoded_data);
            hash.reverse();

            let mut u64_array = [0u64; 4];
            for (i, val) in hash.chunks(8).enumerate() {
                let arr: [u8; 8] = val.try_into().expect("error &[u8] to [u8;8]");
                let chunk = u64::from_le_bytes(arr);
                u64_array[i] = chunk;
            }
            let big_int = BigInt(u64_array);
            Ok(Fr::from(big_int))
        })
        .collect()
}

pub fn srs(path: &str, transcript_index: usize) -> PublicParameter {
    let json_string = fs::read_to_string(path).expect("Unable to read file");

    let srs_json: SRSRaw =
        serde_json::from_str(&json_string).expect("JSON does not have correct format.");

    // let g1 = srs_json
    //     .transcripts[transcript_index]
    //     .powers_of_tau
    //     .g1_power
    //     .iter()
    //     .map(|g1_| Fr::try_from(g1_))
    //     .collect::<Vec<G1Affine>>();

    // let g2 = srs_json
    //     .transcripts[transcript_index]
    //     .powers_of_tau
    //     .g2_power
    //     .iter()
    //     .map(|g2_| G2Affine::try_from(g2_))
    //     .collect::<Vec<G2Affine>>();

    PublicParameter {
        g1: vec![],
        g2: vec![],
    }
}

// pub fn srs_g1() -> Vec<G1Affine> {
//     vec![]
// }

// pub fn srs_g2() -> Vec<G2Affine> {
//     vec![]
// }

#[cfg(test)]
mod tests {
    use super::srs;
    use super::*;
    use ark_bn254::G1Affine;

    #[test]
    fn test_build_srs() {
        let pp = srs("./srs.json", 1);
        let gg = Fr::from_str("0x97f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb");
        match gg {
            Ok(gg) => println!("{:?}", gg),
            Err(e) => println!("{:?}", e),
        }

        println!("{:?}", gg);
    }
}
