use pyo3::prelude::*;
use pyo3::exceptions;
use ::drand_verify::{derive_randomness, G1Pubkey, G2Pubkey, G2PubkeyRfc, Pubkey};

fn do_verify(pk: impl Pubkey, round: u64, previous_signature: &[u8], signature: &[u8]) -> PyResult<String> {
    match pk.verify(round, previous_signature, signature) {
        Err(err) => {
            Err(exceptions::PyValueError::new_err(format!("Error during verification: {}", err)))
        }
        Ok(valid) => {
            if valid {
                let randomness = derive_randomness(&signature);
                Ok(hex::encode(randomness))
            } else {
                Err(exceptions::PyValueError::new_err("Verification Failed"))
            }
        }
    }
}

#[pyfunction]
fn verify_pedersen_bls_chained(round: u64, prev_sig: &str, sig: &str, pk_str: &str) -> PyResult<String> {
    let pk_data = hex::decode(pk_str).unwrap();
    let pk = G1Pubkey::from_variable(pk_data.as_slice()).unwrap();

    let previous_signature = hex::decode(prev_sig).unwrap();
    let signature = hex::decode(sig).unwrap();

    do_verify(pk, round, previous_signature.as_slice(), signature.as_slice())
}

// No alias for this as the fastnet is deprecated
#[pyfunction]
fn verify_bls_unchained_on_g1(round: u64, sig: &str, pk_str: &str) -> PyResult<String> {
    let pk_data = hex::decode(pk_str).unwrap();
    let pk = G2Pubkey::from_variable(pk_data.as_slice()).unwrap();

    let signature = hex::decode(sig).unwrap();

    do_verify(pk, round, &[], signature.as_slice())
}

#[pyfunction]
fn verify_bls_unchained_g1_rfc9380(round: u64, sig: &str, pk_str: &str) -> PyResult<String> {
    let pk_data = hex::decode(pk_str).unwrap();
    let pk = G2PubkeyRfc::from_variable(pk_data.as_slice()).unwrap();

    let signature = hex::decode(sig).unwrap();

    do_verify(pk, round, &[], signature.as_slice())
}

#[pyfunction]
#[pyo3(signature = (round, sig, pk_str="83cf0f2896adee7eb8b5f01fcad3912212c437e0073e911fb90022d3e760183c8c4b450b6a0a6c3ac6a5776a2d1064510d1fec758c921cc22b0e17e63aaf4bcb5ed66304de9cf809bd274ca73bab4af5a6e9c76a4bc09e76eae8991ef5ece45a"))]
fn verify_quicknet(round: u64, sig: &str, pk_str: &str) -> PyResult<String> {
    return verify_bls_unchained_g1_rfc9380(round, sig, pk_str)
}

#[pyfunction]
#[pyo3(signature = (round, prev_sig, sig, pk_str="868f005eb8e6e4ca0a47c8a77ceaa5309a47978a7c71bc5cce96366b5d7a569937c529eeda66c7293784a9402801af31"))]
fn verify_mainnet(round: u64, prev_sig: &str, sig: &str, pk_str: &str) -> PyResult<String> {
    return verify_pedersen_bls_chained(round, prev_sig, sig, pk_str)
}

#[pymodule]
fn drand_verify(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(verify_bls_unchained_g1_rfc9380, m)?)?;
    m.add_function(wrap_pyfunction!(verify_quicknet, m)?)?;
    m.add_function(wrap_pyfunction!(verify_pedersen_bls_chained, m)?)?;
    m.add_function(wrap_pyfunction!(verify_mainnet, m)?)?;
    m.add_function(wrap_pyfunction!(verify_bls_unchained_on_g1, m)?)?;
    Ok(())
}