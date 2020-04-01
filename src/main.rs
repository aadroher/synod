extern crate num;
extern crate num_iter;
extern crate primal;
extern crate rand;

use num::{pow::pow, BigUint, Integer};
use num_iter::range;
use primal::StreamingSieve;
use rand::{thread_rng, Rng};

const P_NTH_INT: usize = 10;
const Q_NTH_INT: usize = 11;
const G_RND_RANGE_UPPER_BOUND: usize = 100;

fn get_prime(nth: usize) -> BigUint {
    let prime = StreamingSieve::nth_prime(nth);
    BigUint::from(prime)
}

fn get_g() -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0, G_RND_RANGE_UPPER_BOUND)
}

fn get_z_n(n: &BigUint) -> Vec<BigUint> {
    let bu1 = BigUint::from(1usize);
    println!("{:?}", bu1);
    range(bu1.clone(), n.clone())
        .filter(|x| x.gcd(n) == bu1)
        .collect()
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

    let n = &(p * q);

    let lambda = (p - 1usize).lcm(&(p - 1usize));
    let g = get_g();
    let z_n_squared = get_z_n(&(n * n));

    println!(
        "p={}, q={}, n={}, lambda={}, g={}, z_n_squared=",
        p,
        q,
        n,
        lambda,
        g,
        // z_n_squared
    );

    // let mu = get_mu(g, lambda, n);

    // println!(
    //     "p={}, q={}, n={}, lambda={}, g={}, mu={}",
    //     p, q, n, lambda, g, mu
    // );
}
