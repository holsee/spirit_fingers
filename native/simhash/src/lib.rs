use rustler::{Atom, Error};

extern crate simhash;

mod atoms {
    rustler::atoms! {
        ok,
        error
    }
}

#[rustler::nif]
fn similarity_hash(text: &str) -> Result<(Atom, u64), Error> {
    let hash: u64 = simhash::simhash(text);
    Ok((atoms::ok(), hash))
}

#[rustler::nif]
fn hamming_distance(hash0: u64, hash1: u64) -> Result<(Atom, u32), Error> {
    let ham_dist: u32 = simhash::hamming_distance(hash0, hash1);
    Ok((atoms::ok(), ham_dist))
}

#[rustler::nif]
fn hash_similarity(hash0: u64, hash1: u64) -> Result<(Atom, f64), Error> {
    let hash_similarity = simhash::hash_similarity(hash0, hash1);
    Ok((atoms::ok(), hash_similarity))
}

#[rustler::nif]
fn similarity(text0: &str, text1: &str) -> Result<(Atom, f64), Error> {
    let similarity: f64 = simhash::similarity(text0, text1);
    Ok((atoms::ok(), similarity))
}

rustler::init!(
    "Elixir.SpiritFingers.SimHash",
    [
        similarity_hash,
        hamming_distance,
        hash_similarity,
        similarity
    ]
);
