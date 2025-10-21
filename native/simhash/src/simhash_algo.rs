// Copyright (c) 2014 Bart Olsthoorn
// Copyright (c) 2017 Jakub Pastuszek
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

//! Rust Simhash implementation
//!
//! Originally implemented by Bart Olsthoorn on 12/08/2014
//! Ported to Rust 1.16.0 by Jakub Pastuszek on 29/05/2017
//! With the help of http://matpalm.com/resemblance/simhash/
//!
//! Vendored from https://github.com/bartolsthoorn/simhash-rs into spirit_fingers project
//! Enhanced with performance improvements and idiomatic Rust patterns by @holsee

use siphasher::sip::SipHasher;
use std::hash::{Hash, Hasher};

/// Number of bits in the hash (u64)
const HASH_BITS: usize = 64;

fn hash_feature<T: Hash>(t: &T) -> u64 {
    let mut s = SipHasher::default();
    t.hash(&mut s);
    s.finish()
}

/// Calculate `u64` simhash from stream of `&str` words
///
/// # Returns
/// Returns `0` for empty input streams
pub fn simhash_stream<'w, W>(words: W) -> u64
where
    W: Iterator<Item = &'w str>,
{
    let mut v = [0i32; HASH_BITS];
    let mut simhash: u64 = 0;

    for feature in words {
        let feature_hash: u64 = hash_feature(&feature);

        // Update weights for each bit position
        for (i, weight) in v.iter_mut().enumerate() {
            let bit = (feature_hash >> i) & 1;
            if bit == 1 {
                *weight = weight.saturating_add(1);
            } else {
                *weight = weight.saturating_sub(1);
            }
        }
    }

    // Build final hash from positive weights
    for (i, &weight) in v.iter().enumerate() {
        if weight > 0 {
            simhash |= 1 << i;
        }
    }
    simhash
}

/// Calculate `u64` simhash from `&str` split by whitespace
///
/// # Examples
/// ```
/// # use spirit_fingers_simhash::simhash_algo::simhash;
/// let hash = simhash("The cat sat on the mat");
/// assert_eq!(hash, 2595200813813010837);
/// ```
///
/// # Returns
/// Returns `0` for empty or whitespace-only strings
pub fn simhash(text: &str) -> u64 {
    simhash_stream(text.split_whitespace())
}

/// Bitwise hamming distance of two `u64` hashes
pub fn hamming_distance(x: u64, y: u64) -> u32 {
    (x ^ y).count_ones()
}

/// Calculate similarity as `f64` of two hashes
/// 0.0 means no similarity, 1.0 means identical
pub fn hash_similarity(hash1: u64, hash2: u64) -> f64 {
    let distance: f64 = hamming_distance(hash1, hash2) as f64;
    1.0 - (distance / HASH_BITS as f64)
}

/// Calculate similarity of two streams of string slices by simhash
pub fn similarity_streams<'w1, 'w2, W1, W2>(words1: W1, words2: W2) -> f64
where
    W1: Iterator<Item = &'w1 str>,
    W2: Iterator<Item = &'w2 str>,
{
    hash_similarity(simhash_stream(words1), simhash_stream(words2))
}

/// Calculate similarity of two string slices split by whitespace by simhash
pub fn similarity(text1: &str, text2: &str) -> f64 {
    similarity_streams(text1.split_whitespace(), text2.split_whitespace())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simhash_test() {
        assert_eq!(simhash("The cat sat on the mat"), 2595200813813010837);
        assert_eq!(simhash("The cat sat under the mat"), 2595269945604666783);
        assert_eq!(simhash("Why the lucky stiff"), 1155526875459215761);
    }

    #[test]
    fn hamming_distance_test() {
        assert_eq!(hamming_distance(0b0000000u64, 0b0000000u64), 0);
        assert_eq!(hamming_distance(0b1111111u64, 0b0000000u64), 7);
        assert_eq!(hamming_distance(0b0100101u64, 0b1100110u64), 3);
    }

    #[test]
    fn hash_similarity_test() {
        assert_eq!(hash_similarity(0u64, 0u64), 1.0);
        assert_eq!(hash_similarity(!0u64, 0u64), 0.0);
        assert_eq!(hash_similarity(!0u32 as u64, 0u64), 0.5);
    }

    #[test]
    fn similarity_test() {
        assert_eq!(similarity("Stop hammertime", "Stop hammertime"), 1.0);
        assert!(similarity("Hocus pocus", "Hocus pocus pilatus pas") > 0.7);
        assert!(similarity("Peanut butter", "Strawberry cocktail") < 0.6);
    }
}
