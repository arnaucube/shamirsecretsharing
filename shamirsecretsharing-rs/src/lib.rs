extern crate rand;
extern crate num;
extern crate num_bigint;
extern crate num_traits;

use num_bigint::RandBigInt;
use num::pow::pow;


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

fn mod_inverse(a: BigInt, module: BigInt) -> BigInt {
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
        let modinv = mod_inverse(res_d, p.clone());
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
}
