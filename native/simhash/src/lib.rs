// Copyright (c) 2023 Steven Holdsworth
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use rustler::{Atom, Error};

mod simhash_algo;

mod atoms {
    rustler::atoms! {
        ok,
        error
    }
}

#[rustler::nif]
fn similarity_hash(text: &str) -> Result<(Atom, u64), Error> {
    let hash: u64 = simhash_algo::simhash(text);
    Ok((atoms::ok(), hash))
}

#[rustler::nif]
fn hamming_distance(hash0: u64, hash1: u64) -> Result<(Atom, u32), Error> {
    let ham_dist: u32 = simhash_algo::hamming_distance(hash0, hash1);
    Ok((atoms::ok(), ham_dist))
}

#[rustler::nif]
fn hash_similarity(hash0: u64, hash1: u64) -> Result<(Atom, f64), Error> {
    let hash_similarity = simhash_algo::hash_similarity(hash0, hash1);
    Ok((atoms::ok(), hash_similarity))
}

#[rustler::nif]
fn similarity(text0: &str, text1: &str) -> Result<(Atom, f64), Error> {
    let similarity: f64 = simhash_algo::similarity(text0, text1);
    Ok((atoms::ok(), similarity))
}

rustler::init!("Elixir.SpiritFingers.SimHash");
