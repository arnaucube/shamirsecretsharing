# go-shamirsecretsharing [![Go Report Card](https://goreportcard.com/badge/github.com/arnaucube/shamirsecretsharing)](https://goreportcard.com/report/github.com/arnaucube/shamirsecretsharing) [![GoDoc](https://godoc.org/github.com/arnaucube/shamirsecretsharing?status.svg)](https://godoc.org/github.com/arnaucube/shamirsecretsharing)
Shamir's Secret Sharing in Go lib + WASM lib

This directory contains a Go lang implementation of Shamir's Secret Sharing, and a compiled Web Assembly version from the Go code to be used from the browser.

- https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing

## Wasm usage
Compile to wasm, inside the `wasm` directory, execute:
```
GOARCH=wasm GOOS=js go build -o shamirsecretsharing.wasm shamirsecretsharing-wasm-wrapper.go
```

Add the file `wasm_exec.js` in the directory:
```
cp "$(go env GOROOT)/misc/wasm/wasm_exec.js" .
```

Call the library from javascript:
```js
// Create shares from a secret
// nNeededShares: number of secrets needed
// nShares: number of shares
// p: random point
// k: secret to share
createShares(nNeededShares, nShares, p, k);
```

## Usage from Go
```go
// define secret to share
k, ok := new(big.Int).SetString("123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890", 10)
assert.True(t, ok)

// define random prime
p, err := rand.Prime(rand.Reader, bits/2)
assert.Nil(t, err)

// define how many shares want to generate
nShares := big.NewInt(int64(6))

// define how many shares are needed to recover the secret
nNeededShares := big.NewInt(int64(3))

// create the shares
shares, err := Create(
	nNeededShares,
	nShares,
	p,
	big.NewInt(int64(k)))
assert.Nil(t, err)

// select shares to use
var sharesToUse [][]*big.Int
sharesToUse = append(sharesToUse, shares[2])
sharesToUse = append(sharesToUse, shares[1])
sharesToUse = append(sharesToUse, shares[0])

// recover the secret using Lagrange Interpolation
secr := LagrangeInterpolation(sharesToUse, p)

// check that the restored secret matches the original secret
if !bytes.Equal(k.Bytes(), secr.Bytes()) {
	fmt.Println("reconstructed secret not correspond to original secret")
}
```
