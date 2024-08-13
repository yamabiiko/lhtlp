[![Crates.io](https://img.shields.io/crates/v/lhtlp?style=flat-square)](https://crates.io/crates/lhtlp)
![Crates.io](https://img.shields.io/crates/l/lhtlp?style=flat-square)

# Linearly Homorphic TimeLock Puzzles (LHTLP) Implementation in Rust

This LHTLP implementation is written purely in Rust, building on top of `num-bigint` and `num-primes` crates.

It implements the protocol described in Section 4.1 of [Homomorphic Time-Lock Puzzles and Applications](https://eprint.iacr.org/2019/635.pdf), Malavolta and Thyagarajan, 2019.

## What is a LHTLP?

A LHTLP is a linearly homomorphic time-lock puzzles. Time-lock puzzles are cryptographic primitives that allow to encrypt a secret in a puzzle that can only be recovered after performing a certain amount of operations that are inherently sequential (squaring in RSA groups). The linearly homorphic properties imply that a set of puzzles can be evaluated homomorphically, that is a set of puzzles of can be bundled up together or evaluated with a circuit as a single puzzle. See [https://eprint.iacr.org/2019/635.pdf](https://eprint.iacr.org/2019/635.pdf) for more details.

## Usage
### Setup, generate and solve a puzzle
Setting up a LHTLP requires 2 parameters:
* _lambda_: security parameters that sets the number of bits of the randomly generated safe primes
* _difficulty_: number of iterations to perform, linearly increasing computation time when retrieving the secret with `solve`
```rust
 use lhltp::LHTLP;
 const difficulty: u64 = 100000000;
 const lambda: u64 = 64;

 let lhtlp = LHTLP::setup(lambda, BigUint::from(difficulty));
 let secret = 42;
 let puzzle = lhtlp.generate(secret);
 let solution = lhtlp:solve(puzzle);
```
### Homomorphic evaluation of multiple puzzles
```rust
 let first = lhtlp.generate(42);
 let second = lhtlp.generate(13);
 let bundle = lhtlp.eval(vec![first, second]);
 let solution = lhtlp:solve(puzzle);

 assert!(BigUint::from(55u32), solution);
```
