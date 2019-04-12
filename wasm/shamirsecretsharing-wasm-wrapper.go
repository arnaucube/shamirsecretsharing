package main

import (
	"math/big"
	"syscall/js"

	"github.com/arnaucube/shamirsecretsharing"
)

func main() {
	c := make(chan struct{}, 0)

	println("WASM Go Initialized")
	// register functions
	registerCallbacks()
	<-c
}

func registerCallbacks() {
	js.Global().Set("createShares", js.ValueOf(createShares))
	js.Global().Set("lagrangeInterpolation", js.ValueOf(lagrangeInterpolation))
}

func createShares(i []js.Value) {
	nNeededShares, ok := new(big.Int).SetString(i[0].String(), 10)
	if !ok {
		println("error parsing parameter in position 0")
	}
	nShares, ok := new(big.Int).SetString(i[1].String(), 10)
	if !ok {
		println("error parsing parameter in position 1")
	}
	p, ok := new(big.Int).SetString(i[2].String(), 10)
	if !ok {
		println("error parsing parameter in position 2")
	}
	k, ok := new(big.Int).SetString(i[3].String(), 10)
	if !ok {
		println("error parsing parameter in position 3")
	}
	shares, err := shamirsecretsharing.Create(nNeededShares, nShares, p, k)
	if err != nil {
		println("error generating the shares")
	}
	println(shares)
	sharesStr := sharesToString(shares)
	println(sharesStr)
}

func sharesToString(shares [][]*big.Int) []string {
	var s []string
	for i := 0; i < len(shares); i++ {
		s = append(s, shares[i][0].String())
		s = append(s, shares[i][1].String())
	}
	return s
}

func lagrangeInterpolation(i []js.Value) {
	p, ok := new(big.Int).SetString(i[0].String(), 10)
	if !ok {
		println("error parsing parameter in position 0 (p)")
	}

	// parse the shares array
	var shares [][]*big.Int
	for n := 1; n < len(i); n = n + 2 {
		a, ok := new(big.Int).SetString(i[n].String(), 10)
		if !ok {
			println("error parsing parameter in position ", n)
		}
		b, ok := new(big.Int).SetString(i[n+1].String(), 10)
		if !ok {
			println("error parsing parameter in position ", n+1)
		}
		var share []*big.Int
		share = append(share, a)
		share = append(share, b)
		shares = append(shares, share)
	}

	secr := shamirsecretsharing.LagrangeInterpolation(p, shares)
	println(secr.String())
}
