extern crate rand;
extern crate num;
extern crate num_bigint;
extern crate num_traits;

use std::str::FromStr;

use num_bigint::RandBigInt;
use num::pow::pow;
use num::Integer;


use num_bigint::{BigInt, ToBigInt};
use num_traits::{Zero, One};

fn modulus(a: &BigInt, m: &BigInt) -> BigInt {
    ((a%m) + m) % m
}

pub fn create(t: u32, n: u32,p: &BigInt, k: &BigInt) -> Vec<[BigInt;2]> {
    // t: number of secrets needed
    // n: number of shares
    // p: random point
    // k: secret to share
    if k>p {
        println!("\nERROR: need k<p\n");
    }

    // generate base_polynomial
    let mut base_polynomial: Vec<BigInt> = Vec::new();
    base_polynomial.push(k.clone());
    for _ in 0..t as usize-1 {
        let mut rng = rand::thread_rng();
        let a = rng.gen_bigint(1024);
        base_polynomial.push(a);
    }

    // calculate shares, based on the base_polynomial
    let mut shares: Vec<BigInt> = Vec::new();
    for i in 1..n+1 {
        let mut p_res: BigInt = Zero::zero();
        let mut x = 0;
        for pol_elem in &base_polynomial {
            if x==0 {
                p_res = p_res + pol_elem;
            } else {
                let i_pow = pow(i, x);
                let curr_elem = i_pow * pol_elem;
                p_res = p_res + curr_elem;
                p_res = modulus(&p_res, p);
            }
            x = x+1;
        }
        shares.push(p_res);
    }
    pack_shares(shares)
}

fn pack_shares(shares: Vec<BigInt>) -> Vec<[BigInt;2]> {
    let mut r: Vec<[BigInt;2]> = Vec::new();
    for i in 0..shares.len() {
        let curr: [BigInt;2] = [shares[i].clone(), (i+1).to_bigint().unwrap()];
        r.push(curr);
    }
    r
}

fn unpack_shares(s: Vec<[BigInt;2]>) -> (Vec<BigInt>, Vec<BigInt>) {
    let mut shares: Vec<BigInt> = Vec::new();
    let mut is: Vec<BigInt> = Vec::new();
    for i in 0..s.len() {
        shares.push(s[i][0].clone());
        is.push(s[i][1].clone());
    }
    (shares, is)
}

pub fn mod_inverse(a: BigInt, module: BigInt) -> BigInt {
    // TODO search biguint impl of mod_inv
    let mut mn = (module.clone(), a);
    let mut xy: (BigInt, BigInt) = (Zero::zero(), One::one());

    let big_zero: BigInt = Zero::zero();
    while mn.1 != big_zero {
        xy = (xy.1.clone(), xy.0 - (mn.0.clone() / mn.1.clone()) * xy.1);
        mn = (mn.1.clone(), modulus(&mn.0, &mn.1));
    }

    while xy.0 < Zero::zero() {
        xy.0 += module.clone();
    }
    xy.0
}

/// Compute `a^-1 (mod l)` using the the Kalinski implementation
/// of the Montgomery Modular Inverse algorithm.
/// B. S. Kaliski Jr. - The  Montgomery  inverse  and  its  applica-tions.
/// IEEE Transactions on Computers, 44(8):1064–1065, August-1995
pub fn kalinski_inv(a: &BigInt, modulo: &BigInt) -> BigInt {
    // This Phase I indeed is the Binary GCD algorithm , a version o Stein's algorithm
    // which tries to remove the expensive division operation away from the Classical
    // Euclidean GDC algorithm replacing it for Bit-shifting, subtraction and comparaison.
    // 
    // Output = `a^(-1) * 2^k (mod l)` where `k = log2(modulo) == Number of bits`.
    // 
    // Stein, J.: Computational problems associated with Racah algebra.J. Comput. Phys.1, 397–405 (1967).
    let phase1 = |a: &BigInt| -> (BigInt, u64) {
        assert!(a != &BigInt::zero());
        let p = modulo;
        let mut u = modulo.clone();
        let mut v = a.clone();
        let mut r = BigInt::zero();
        let mut s = BigInt::one();
        let mut k = 0u64;

        while v > BigInt::zero() {
            match(u.is_even(), v.is_even(), u > v, v >= u) {
                // u is even
                (true, _, _, _) => {

                    u = u >> 1;
                    s = s << 1;
                },
                // u isn't even but v is even
                (false, true, _, _) => {

                    v = v >> 1;
                    r = &r << 1;
                },
                // u and v aren't even and u > v
                (false, false, true, _) => {

                    u = &u - &v;
                    u = u >> 1;
                    r = &r + &s;
                    s = &s << 1;
                },
                // u and v aren't even and v > u
                (false, false, false, true) => {

                    v = &v - &u;
                    v = v >> 1;
                    s = &r + &s;
                    r = &r << 1;
                },
                (false, false, false, false) => panic!("Unexpected error has ocurred."),
            }
            k += 1;
        }
        if &r > p {
            r = &r - p;
        }
        ((p - &r), k)
    };

    // Phase II performs some adjustments to obtain
    // the Montgomery inverse.
    // 
    // We implement it as a clousure to be able to grap the 
    // kalinski_inv scope to get `modulo` variable.
    let phase2 = |r: &BigInt, k: &u64| -> BigInt {
        let mut rr = r.clone();
        let _p = modulo;

        for _i in 0..*k {
            match rr.is_even() {
                true => {
                    rr = rr >> 1;
                },
                false => {
                    rr = (rr + modulo) >> 1;
                }
            }
        }
        rr 
    };

    let (r, z) = phase1(&a.clone());

    phase2(&r, &z)
}

pub fn lagrange_interpolation(p: &BigInt, shares_packed: Vec<[BigInt;2]>) -> BigInt {
    let mut res_n: BigInt = Zero::zero();
    let mut res_d: BigInt = Zero::zero();
    let (shares, sh_i) = unpack_shares(shares_packed);

    for i in 0..shares.len() {
        let mut lagrange_numerator: BigInt = One::one();
        let mut lagrange_denominator: BigInt = One::one();
        for j in 0..shares.len() {
            if shares[i] != shares[j] {
                let curr_l_numerator = &sh_i[j];
                let curr_l_denominator = &sh_i[j] - &sh_i[i];
                lagrange_numerator = lagrange_numerator * curr_l_numerator;
                lagrange_denominator = lagrange_denominator * curr_l_denominator;
            }
        }
        let numerator: BigInt = &shares[i] * &lagrange_numerator;

        let quo: BigInt = (&numerator / &lagrange_denominator) + (&lagrange_denominator ) % &lagrange_denominator;
        if quo != Zero::zero() {
            res_n = res_n + quo;
        } else {
            let res_n_mul_lagrange_den = res_n * &lagrange_denominator;
            res_n = res_n_mul_lagrange_den + numerator;
            res_d = res_d + lagrange_denominator;
        }
    }
    let modinv_mul: BigInt;
    if res_d != Zero::zero() {
        let modinv = kalinski_inv(&res_d, &p);
        modinv_mul = res_n * modinv;
    } else {
        modinv_mul = res_n;
    }
    let r = modulus(&modinv_mul, &p);
    r
}


#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_create_and_lagrange_interpolation() {
        let mut rng = rand::thread_rng();
        let p = rng.gen_biguint(1024).to_bigint().unwrap();
        println!("p: {:?}", p);
        let k = BigInt::parse_bytes(b"123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890", 10).unwrap();

        let s = create(3, 6, &p, &k);
        // println!("s: {:?}", s);

        let mut shares_to_use: Vec<[BigInt;2]> = Vec::new();
        shares_to_use.push(s[2].clone());
        shares_to_use.push(s[1].clone());
        shares_to_use.push(s[0].clone());
        let r = lagrange_interpolation(&p, shares_to_use);
        println!("recovered secret: {:?}", r.to_string());
        println!("original secret: {:?}", k.to_string());
        assert_eq!(k, r);
    }

    #[test]
    fn kalinski_modular_inverse() {
        let modul1 = BigInt::from(127u64);

        let a = BigInt::from(79u64);
        let res1 = kalinski_inv(&a, &modul1);
        let expected1 = BigInt::from(82u64);
        assert_eq!(res1, expected1);

        let b = BigInt::from(50u64);
        let res2 = kalinski_inv(&b, &modul1);
        let expected2 = BigInt::from(94u64);
        assert_eq!(res2, expected2);

        // Modulo: 2^252 + 27742317777372353535851937790883648493
        // Tested: 182687704666362864775460604089535377456991567872
        // Expected for: inverse_mod(a, l) computed on SageMath:
        // `7155219595916845557842258654134856828180378438239419449390401977965479867845`.
        let modul3 = BigInt::from_str("7237005577332262213973186563042994240857116359379907606001950938285454250989").unwrap();
        let d = BigInt::from_str("182687704666362864775460604089535377456991567872").unwrap();
        let res4 = kalinski_inv(&d, &modul3); 
        let expected4 = BigInt::from_str("7155219595916845557842258654134856828180378438239419449390401977965479867845").unwrap();
        assert_eq!(expected4, res4);
    }
}
