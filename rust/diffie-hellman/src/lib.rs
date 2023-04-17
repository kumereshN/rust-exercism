use rand::thread_rng;
use rand::seq::SliceRandom;

fn powmod(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }
    result
}

pub fn is_prime(n: u64) -> bool {
    let sqrt = (n as f32).sqrt() as u64;
    (2..=sqrt)
        .all(|num| n % num != 0)
}

pub fn private_key(p: u64) -> u64 {
    let mut rng = thread_rng();

    *(2..=p-1)
        .filter(|&n| is_prime(n))
        .collect::<Vec<u64>>()
        .choose(&mut rng)
        .unwrap()
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    powmod(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    powmod(b_pub, a, p)
}
