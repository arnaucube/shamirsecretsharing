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
}

func lagrangeInterpolation(i []js.Value) {
}
