extern crate num;
extern crate rand;

use num::integer::lcm;

const P_NTH_INT: usize = 10000;
const Q_NTH_INT: usize = 10050;

fn get_prime(nth: usize) -> Option<usize> {
    let (_lo, hi) = slow_primes::estimate_nth_prime(10002);
    let sieve = slow_primes::Primes::sieve(hi as usize);

    sieve.primes().nth(nth - 1)
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

    println!("p={}, q={}, n={}, lambda={}", p, q, n, lambda);
}
