//! # lhtlp -  Linearly Homomorphic TimeLock Puzzles (LHTLP) implementation
//! This crate provides a simple library implementation of LHTLP in pure Rust.
//! ## Setup, generate and solve a puzzle
//! ```rust
//! use lhltp::LHTLP;
//! const TIME_HARDNESS: u64 = 100000000;
//!
//! let lhtlp = LHTLP::setup(64, BigUint::from(TIME_HARDNESS));
//! let secret = 42;
//! let puzzle = lhtlp.generate(secret);
//! let solution = lhtlp:solve(puzzle);
//! ```
//! ## Homomorphic evaluation of multiple puzzles
//! ```rust
//! let first = lhtlp.generate(42);
//! let second = lhtlp.generate(13);
//! let bundle = lhtlp.eval(vec![first, second]);
//! let solution = lhtlp:solve(puzzle);
//!
//! assert!(BigUint::from(55u32), solution);
//! ```
//!
pub mod num_primes;

use crate::num_primes::{Generator, RandBigInt};
use num_bigint::BigUint;
use num_traits::pow::Pow;
use num_integer::Integer;

/// A Linearly Homomorphic Timelock Puzzle.
///
/// A LHTLP is a linearly homomorphic version of time-lock puzzles, which are cryptographic primitives that
/// allow to encrypt a secret in a puzzle that can only be recovered after performing a certain
/// amount of sequential operations.
#[derive(Debug, Clone)]
pub struct LHTLP {
    difficulty: BigUint,
    n: BigUint,
    g: BigUint,
    h: BigUint
}

impl LHTLP {
    /// Setup an instance of a LHTLP based on time and security parameter.
    ///
    /// The security parameter `lambda` sets the number of bits of the randomly generated safe primes. \
    /// Setting `difficulty` to 100000000 results in roughly 5 seconds of computation when
    /// opening a puzzle with `solve`.
    pub fn setup(lambda: u64, difficulty: BigUint) -> LHTLP {
        let p = Generator::safe_prime(lambda);
        let q = Generator::safe_prime(lambda);

        let n = &p * &q;
        let one = BigUint::from(1u32);
        let two = BigUint::from(2u32);

        let mut rng = rand::thread_rng();
        let g = loop {
            let rand = rng.gen_biguint_range(&one, &n);
            if rand.gcd(&n) == one {
                break rand;
            }
        };

        let g = &g.pow(&two);
        let g = g.modinv(&n).unwrap();

        // phi(n) = p-1 * q-1
        // phi(n) / 2
        let tot_div_2 = (&p-&one) * (&q-&one) / &BigUint::from(2u32);
        let pow = &two.modpow(&difficulty, &tot_div_2);
        let h = g.modpow(pow, &n);

        LHTLP { 
            difficulty,
            n,
            g,
            h,
        }
    }

    /// Generate a puzzle `(u: BigUint, v: BigUint)` embedding a `secret` value.
    ///
    pub fn generate(&self, secret: u64) -> (BigUint, BigUint) {
        let mut rng = rand::thread_rng();
        let n2 = (&self.n).pow(2u32);
        let one = BigUint::from(1u32);
        let r = rng.gen_biguint_range(&one, &n2);
        let u = self.g.modpow(&r, &self.n);
        let v = ((&self.h).modpow(&(&r * &self.n), &n2) * (&one + &self.n).modpow(&BigUint::from(secret), &n2)) % n2;
        (u, v)
    }

    /// Open a puzzle `(u: BigUint, v: BigUint)` by performing sequential squaring, revealing a `secret` value.
    ///
    pub fn solve(&self, puzzle: (BigUint, BigUint)) -> BigUint {
        let n2 = (&self.n).pow(2u32);
        let w = puzzle.0.modpow(&BigUint::from(2u32).pow(&self.difficulty), &self.n);
        let s = ((&puzzle.1 * w.modpow(&self.n, &n2).modinv(&n2).unwrap()) % (&self.n).pow(2u32) -  BigUint::from(1u32))/ &self.n;
        s
    }

    /// Linearly homomorphic evaluate a vector of puzzles.
    ///
    /// The resulting puzzle embeds a secret equivalent to the sum of the secrets embedded in the single puzzles.
    pub fn evaluate(&self, puzzles: Vec<(BigUint, BigUint)>) -> (BigUint, BigUint) {
        let one = BigUint::from(1u32);
        puzzles.iter().fold((one.clone(), one), |acc, x| ((acc.0 * &x.0), (acc.1 * &x.1)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    // Roughly 5 sec, increases linearly
    const DIFFICULTY: u64 = 100000000;
    const LAMBDA: u64 = 64;
    
    #[test]
    fn gen_and_solve() {
        let mut rng = rand::thread_rng();
        let secret: u64 = rng.gen();
        let lhtlp = LHTLP::setup(LAMBDA, BigUint::from(DIFFICULTY));
        let puzzle = lhtlp.generate(secret);
        let result = lhtlp.solve(puzzle);
        assert!(BigUint::from(secret) == result);
    }

    #[test]
    fn evaluate() {
        let lhtlp = LHTLP::setup(LAMBDA, BigUint::from(DIFFICULTY));
        let mut rng = rand::thread_rng();
        let mut secrets: Vec<u64> = Vec::new();
        let mut puzzles: Vec<(BigUint, BigUint)> = Vec::new();
        let mut solution = BigUint::from(0u32);
        for _i in 0..40 {
            let secret: u64 = rng.gen();
            secrets.push(secret);
            let puzzle = lhtlp.generate(secret);
            puzzles.push(puzzle);
            solution += BigUint::from(secret);

        };
        let eval_puzzle = lhtlp.evaluate(puzzles);
        let result = lhtlp.solve(eval_puzzle);

        assert!(result == solution);
    }

}
