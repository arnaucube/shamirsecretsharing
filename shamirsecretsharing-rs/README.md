# shamirsecretsharing-rs
Shamir's Secret Sharing in Rust


## Usage
```rust
// create 6 shares from k, on the Fp
// where to recover will be needed 3 shares
let s = create(3, 6, &p, &k);

// now set 3 of the 6 shares, to be used to recover the secret
let mut shares_to_use: Vec<[BigInt;2]> = Vec::new();
shares_to_use.push(s[2].clone());
shares_to_use.push(s[1].clone());
shares_to_use.push(s[0].clone());

// recover the secret using Lagrange Interpolation
let r = lagrange_interpolation(&p, shares_to_use);
assert_eq!(k, r);
// r is the secret recovered (k)
```
