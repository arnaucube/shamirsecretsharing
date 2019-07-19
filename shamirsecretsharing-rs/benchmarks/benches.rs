#[macro_use]
extern crate criterion;
extern crate shamirsecretsharing_rs;
extern crate num_bigint;

use criterion::{Criterion, Benchmark};
use shamirsecretsharing_rs::*;
use num_bigint::BigInt;

use std::str::FromStr;


mod mod_inv_benches {
    use super::*;

    pub fn bench_modular_inv(c: &mut Criterion) {

        let modul1 = BigInt::from_str("7237005577332262213973186563042994240857116359379907606001950938285454250989").unwrap();
        let d1 = BigInt::from_str("182687704666362864775460604089535377456991567872").unwrap();

        let modul2 = BigInt::from_str("7237005577332262213973186563042994240857116359379907606001950938285454250989").unwrap();
        let d2 = BigInt::from_str("182687704666362864775460604089535377456991567872").unwrap();

        c.bench(
            "Modular Inverse",
            Benchmark::new("Kalinski Modular inverse", move |b| b.iter(|| kalinski_inv(&d1, &modul1)))
        );

        c.bench(
            "Modular Inverse",
            Benchmark::new("Standard Mod Inv", move |b| b.iter(|| mod_inverse(d2.clone(), modul2.clone())))
        );
    }
}

criterion_group!(benches,
                mod_inv_benches::bench_modular_inv);
criterion_main!(benches);