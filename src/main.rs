extern crate num;

const P_NTH_INT: usize = 10000;
const Q_NTH_INT: usize = 10050;

fn get_prime(nth: usize) -> Option<usize> {
    let (_lo, hi) = slow_primes::estimate_nth_prime(10002);
    let sieve = slow_primes::Primes::sieve(hi as usize);

    sieve.primes().nth(nth - 1)
}

fn main() {
    match get_prime(P_NTH_INT) {
        Some(p) => println!("p is {}", p),
        None => unreachable!(),
    }

    match get_prime(Q_NTH_INT) {
        Some(p) => println!("q is {}", p),
        None => unreachable!(),
    }
}
