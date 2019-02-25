#[macro_use] extern crate rustler;
#[macro_use] extern crate lazy_static;

use rustler::{Env, Term, NifResult, Encoder};

extern crate simhash;

mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.SpiritFingers.SimHash",
    [("simhash", 1, simhash),
     ("hamming_distance", 2, hamming_distance),
     ("hash_similarity", 2, hash_similarity),
     ("similarity", 2, similarity)],
    None
}

fn simhash<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let text: &str = try!(args[0].decode());
    let hash: u64 = simhash::simhash(text);
    Ok((atoms::ok(), hash).encode(env))
}

fn hamming_distance<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let hash0: u64 = try!(args[0].decode());
    let hash1: u64 = try!(args[1].decode());
    let ham_dist: f64 = simhash::hamming_distance(hash0, hash1) as f64;
    Ok((atoms::ok(), ham_dist).encode(env))
}

fn hash_similarity<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let hash0: u64 = try!(args[0].decode());
    let hash1: u64 = try!(args[1].decode());
    let similarity: f64 = simhash::hash_similarity(hash0, hash1) as f64;
    Ok((atoms::ok(), similarity).encode(env))
}

fn similarity<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let text0: &str = try!(args[0].decode());
    let text1: &str = try!(args[1].decode());
    let similarity: f64 = simhash::similarity(text0, text1) as f64;
    Ok((atoms::ok(), similarity).encode(env))
}
