extern crate num;
extern crate rand;

use num::{integer::lcm, pow::pow, BigUint};
use num_traits::cast::ToPrimitive;
use rand::{thread_rng, Rng};

const P_NTH_INT: usize = 10;
const Q_NTH_INT: usize = 12;
const G_RND_RANGE_UPPER_BOUND: usize = 100;

fn get_prime(nth: usize) -> Option<usize> {
    let (_lo, hi) = slow_primes::estimate_nth_prime(10002);
    let sieve = slow_primes::Primes::sieve(hi as usize);

    sieve.primes().nth(nth - 1)
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
    let maybe_p = get_prime(P_NTH_INT);
    let maybe_q = get_prime(Q_NTH_INT);

    let (p, q) = match (maybe_p, maybe_q) {
        (Some(p), Some(q)) => (p, q),
        _ => unreachable!(),
    };

    let n = p * q;
    let lambda = lcm(p - 1, q - 1);
    let g = get_g();

    println!("p={}, q={}, n={}, lambda={}, g={}", p, q, n, lambda, g);

    let mu = get_mu(g, lambda, n);

    println!(
        "p={}, q={}, n={}, lambda={}, g={}, mu={}",
        p, q, n, lambda, g, mu
    );
}
