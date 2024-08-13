use num_primes::{Generator, RandBigInt};
use num_bigint::BigUint;
use num_traits::pow::Pow;
use num_integer::Integer;

#[derive(Debug)]
pub struct LHTLP {
    time_hardness: BigUint,
    n: BigUint,
    g: BigUint,
    h: BigUint
}

impl LHTLP {
    pub fn setup(time_hardness: BigUint) -> LHTLP { 
        let p = Generator::safe_prime(64);
        let q = Generator::safe_prime(64);

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
        let pow = &two.modpow(&time_hardness, &tot_div_2); 
        let h = g.modpow(pow, &n);

        LHTLP { 
            time_hardness, 
            n,
            g,
            h,
        }
    }

    pub fn generate(&self, secret: u64) -> (BigUint, BigUint) {
        let mut rng = rand::thread_rng();
        let n2 = (&self.n).pow(2u32);
        let one = BigUint::from(1u32);
        let r = rng.gen_biguint_range(&one, &n2);
        let u = self.g.modpow(&r, &self.n);
        let v = ((&self.h).modpow(&(&r * &self.n), &n2) * (&one + &self.n).modpow(&BigUint::from(secret), &n2)) % n2;
        //println!("u, v {}, {}", u, v);
        (u, v)
    }

    pub fn solve(&self, puzzle: (BigUint, BigUint)) -> BigUint {
        let n2 = (&self.n).pow(2u32);
        let w = puzzle.0.modpow(&BigUint::from(2u32).pow(&self.time_hardness), &self.n);
        let s = ((&puzzle.1 * w.modpow(&self.n, &n2).modinv(&n2).unwrap()) % (&self.n).pow(2u32) -  BigUint::from(1u32))/ &self.n;
        s
    }
    pub fn evaluate(&self, puzzles: Vec<(BigUint, BigUint)>) -> (BigUint, BigUint) {

        let one = BigUint::from(1u32);
        let (u, v): (BigUint, BigUint) = puzzles.iter().fold((one.clone(), one), |acc, x| ((acc.0 * &x.0), (acc.1 * &x.1)));
        (u, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    // Roughly 5 sec, increases linearly
    const TIME_HARDNESS: u64 = 100000000;
    
    #[test]
    fn gen_and_solve() {
        let mut rng = rand::thread_rng();
        let secret: u64 = rng.gen();
        let lhtlp = LHTLP::setup(BigUint::from(TIME_HARDNESS));
        let puzzle = lhtlp.generate(secret);
        let result = lhtlp.solve(puzzle);
        assert!(BigUint::from(secret) == result);
    }

    #[test]
    fn evaluate() {
        let lhtlp = LHTLP::setup(BigUint::from(TIME_HARDNESS));
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
