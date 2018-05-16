#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;

use rustler::{NifEnv, NifTerm, NifResult, NifEncoder};

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
    "Elixir.SimHash",
    [("simhash", 1, simhash),
     ("hamming_distance", 2, hamming_distance),
     ("hash_similarity", 2, hash_similarity)],
    None
}

fn simhash<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let text: &str = try!(args[0].decode());
    let hash: u64 = simhash::simhash(text);
    Ok((atoms::ok(), hash).encode(env))
}

fn hamming_distance<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let hash0: u64 = try!(args[0].decode());
    let hash1: u64 = try!(args[1].decode());
    let ham_dist: f64 = simhash::hamming_distance(hash0, hash1) as f64;
    Ok((atoms::ok(), ham_dist).encode(env))
}

fn hash_similarity<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let hash0: u64 = try!(args[0].decode());
    let hash1: u64 = try!(args[1].decode());
    let similarity: f64 = simhash::hash_similarity(hash0, hash1) as f64;
    Ok((atoms::ok(), similarity).encode(env))
}

