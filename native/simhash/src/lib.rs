#[macro_use] extern crate rustler;

extern crate simhash;

use rustler::{Env, Term, Error, Encoder};

mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
    }
}

rustler::rustler_export_nifs! {
    "Elixir.SpiritFingers.SimHash",
    [("simhash", 1, simhash),
     ("hamming_distance", 2, hamming_distance),
     ("hash_similarity", 2, hash_similarity),
     ("similarity", 2, similarity)],
    None
}

fn simhash<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let text: &str = args[0].decode()?;
    let hash: u64 = simhash::simhash(text);
    Ok((atoms::ok(), hash).encode(env))
}

fn hamming_distance<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let hash0: u64 = args[0].decode()?;
    let hash1: u64 = args[1].decode()?;
    let ham_dist: f64 = simhash::hamming_distance(hash0, hash1) as f64;
    Ok((atoms::ok(), ham_dist).encode(env))
}

fn hash_similarity<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let hash0: u64 = args[0].decode()?;
    let hash1: u64 = args[1].decode()?;
    let similarity: f64 = simhash::hash_similarity(hash0, hash1) as f64;
    Ok((atoms::ok(), similarity).encode(env))
}

fn similarity<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let text0: &str = args[0].decode()?;
    let text1: &str = args[1].decode()?;
    let similarity: f64 = simhash::similarity(text0, text1) as f64;
    Ok((atoms::ok(), similarity).encode(env))
}
