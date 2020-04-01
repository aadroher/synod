extern crate num;
extern crate primal;
extern crate rand;

use num::{pow::pow, BigUint, Integer};
use primal::StreamingSieve;
use rand::{thread_rng, Rng};

const P_NTH_INT: usize = 10001;
const Q_NTH_INT: usize = 10056;
const G_RND_RANGE_UPPER_BOUND: usize = 100;

fn get_prime(nth: usize) -> BigUint {
    let prime = StreamingSieve::nth_prime(nth);
    BigUint::from(prime)
}

fn get_g() -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0, G_RND_RANGE_UPPER_BOUND)
}

fn get_mu(g: usize, lambda: usize, n: usize) -> usize {
    println!("g={}, lambda={}", g, lambda);
    let x = pow(g, lambda) % pow(n, 2);
    let l = (x - 1) / n;
    (1 / l) % n
}

fn main() {
    let p = &get_prime(P_NTH_INT);
    let q = &get_prime(Q_NTH_INT);

    let n = p * q;

    let lambda = (p - 1usize).lcm(&(p - 1usize));
    let g = get_g();

    println!("p={}, q={}, n={}, lambda={}, g={}", p, q, n, lambda, g);

    // let mu = get_mu(g, lambda, n);

    // println!(
    //     "p={}, q={}, n={}, lambda={}, g={}, mu={}",
    //     p, q, n, lambda, g, mu
    // );
}
