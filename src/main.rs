use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt};
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::thread_rng;
use std::str;

/// ------------ MATH UTILS --------------

fn egcd(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if b.is_zero() {
        (a, BigInt::one(), BigInt::zero())
    } else {
        let (g, x, y) = egcd(b.clone(), a.mod_floor(&b));
        (g, y.clone(), x - (a / b) * y)
    }
}

fn modinv(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (g, x, _) = egcd(a.clone(), m.clone());
    if g == BigInt::one() || g == (-BigInt::one()) {
        Some((x % m + m) % m)
    } else {
        None
    }
}

fn is_probable_prime(n: &BigUint, k: u32) -> bool {
    // Miller-Rabin primality test (simple)
    if *n < BigUint::from(2u32) { return false; }
    if *n == BigUint::from(2u32) || *n == BigUint::from(3u32) { return true; }
    if n % 2u32 == BigUint::zero() { return false; }

    // write n-1 as d*2^r
    let one = BigUint::one();
    let two = &one + &one;
    let n_minus_one = n - &one;
    let mut d = n_minus_one.clone();
    let mut r: u32 = 0;
    while &d % &two == BigUint::zero() {
        d /= &two;
        r += 1;
    }

    let mut rng = thread_rng();
    'witness_loop: for _ in 0..k {
        let a = rng.gen_biguint_range(&two, &(n - &two));
        let mut x = a.modpow(&d, n);
        if x == one.clone() || x == n_minus_one {
            continue;
        }
        for _ in 0..(r-1) {
            x = x.modpow(&two, n);
            if x == n_minus_one {
                continue 'witness_loop;
            }
        }
        return false;
    }
    true
}

fn generate_prime(bits: u64) -> BigUint {
    let mut rng = thread_rng();
    loop {
        let mut candidate = rng.gen_biguint(bits);
        // ensure it's odd and has correct bit length
        candidate.set_bit((bits - 1) as u64, true);
candidate.set_bit(0u64, true);

        if is_probable_prime(&candidate, 16) { // 16 rounds ~ safe for demo
            return candidate;
        }
    }
}

/// ------------ RSA CORE --------------

pub struct PublicKey {
    pub n: BigUint,
    pub e: BigUint,
}

pub struct PrivateKey {
    pub n: BigUint,
    pub d: BigUint,
}

pub struct KeyPair {
    pub public: PublicKey,
    pub private: PrivateKey,
}

pub fn generate_keys(bits: u64) -> KeyPair {
    // Generate two distinct primes p and q (each ~bits/2)
    let half = bits / 2;
    let p = generate_prime(half);
    let mut q = generate_prime(half);
    while q == p { q = generate_prime(half); }

    let n = &p * &q;
    let one = BigUint::one();
    let phi = (&p - &one) * (&q - &one);

    // choose e
    let e_candidates = [65537u32, 3u32, 17u32];
    let mut e = BigUint::from(e_candidates[0]);
    for cand in &e_candidates {
        let ec = BigUint::from(*cand);
        if ec.to_bigint().unwrap().gcd(&phi.to_bigint().unwrap()) == BigInt::one() {
            e = ec;
            break;
        }
    }

    // compute d = e^{-1} mod phi
    let e_bigint = e.to_bigint().unwrap();
    let phi_bigint = phi.to_bigint().unwrap();
    let d_bigint = modinv(&e_bigint, &phi_bigint).expect("mod inverse must exist");
    let d = d_bigint.to_biguint().unwrap();

    KeyPair {
        public: PublicKey { n: n.clone(), e: e.clone() },
        private: PrivateKey { n, d },
    }
}

pub fn encrypt(message: &[u8], pk: &PublicKey) -> BigUint {
    let m = BigUint::from_bytes_be(message);
    if m >= pk.n {
        panic!("Mensagem muito grande para este tamanho de chave! Use padding/fragmentação.");
    }
    m.modpow(&pk.e, &pk.n)
}

pub fn decrypt(cipher: &BigUint, sk: &PrivateKey) -> Vec<u8> {
    cipher.modpow(&sk.d, &sk.n).to_bytes_be()
}

fn main() {
    println!("== RSA em Rust (educacional) ==");
    // 1) Gera chaves
    let bits = 512; // 512 para demo (rápido). Use 2048+ em cenários reais.
    println!("Gerando chaves de {} bits ...", bits);
    let kp = generate_keys(bits);
    println!("Chave pública (n, e): ({} bits, e = {})", kp.public.n.bits(), kp.public.e);
    println!("Chave privada (n, d): ({} bits, d = ...oculto)", kp.private.n.bits());

    // 2) Mensagem de exemplo
   let msg = "BERNARDO CRUZEIRO";

    println!("Mensagem original: {}", msg);

    // 3) Criptografa
    let c = encrypt(msg.as_bytes(), &kp.public);
    println!("Cifra (hex): {}", hex::encode(c.to_bytes_be()));

    // 4) Descriptografa
    let m = decrypt(&c, &kp.private);
    let texto = str::from_utf8(&m).unwrap_or("<UTF8 inválido>");
    println!("Mensagem decriptada: {}", texto);
}
