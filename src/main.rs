extern crate num_bigint;
extern crate rand;

use num_bigint::{BigUint, RandomBits};
use primal::StreamingSieve;
use rand::Rng;
use std::collections::BTreeSet;

const P_NTH_INT: usize = 10000;
const Q_NTH_INT: usize = 10001;
const G_RND_RANGE_UPPER_BOUND: usize = 100;

fn get_prime(nth: usize) -> BigUint {
    let prime = StreamingSieve::nth_prime(nth);
    BigUint::from(prime)
}

fn get_g(n: &BigUint, lambda: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();
    let lower_bound = 0.to_bigint().unwrap();
    let upper_bound = (n * n).to_bigint().unwrap();

    upper_bound;
}

// fn get_z_n(n: &BigUint) -> BTreeSet<BigUint> {
//     let bu1 = BigUint::from(1usize);
//     let candidates = range(bu1.clone(), n.clone());

//     // for element in candidates {
//     //     println!("{:?}", element);
//     // }

//     candidates.collect()
// }

// fn get_mu(g: usize, lambda: usize, n: usize) -> usize {
//     println!("g={}, lambda={}", g, lambda);
//     let x = pow(g, lambda) % pow(n, 2);
//     let l = (x - 1) / n;
//     (1 / l) % n
// }

fn main() {
    let p = &get_prime(P_NTH_INT);
    let q = &get_prime(Q_NTH_INT);

    let n = &(p * q);

    let lambda = (p - 1usize).lcm(&(p - 1usize));
    let g = get_g();
    let n_squared = &(n * n);
    // let z_n_squared = get_z_n(n_squared);

    println!(
        "p={}, q={}, n={}, lambda={}, g={}, n_squared={}, 
        z_n_squared_len=",
        p,
        q,
        n,
        lambda,
        g,
        n_squared,
        // z_n_squared.len()
    );

    // let mu = get_mu(g, lambda, n);

    // println!(
    //     "p={}, q={}, n={}, lambda={}, g={}, mu={}",
    //     p, q, n, lambda, g, mu
    // );
}
