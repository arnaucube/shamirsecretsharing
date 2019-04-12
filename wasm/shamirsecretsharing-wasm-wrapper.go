package main

import (
	"crypto/rand"
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
	js.Global().Set("createShares", js.FuncOf(createShares))
	js.Global().Set("lagrangeInterpolation", js.FuncOf(lagrangeInterpolation))
}

func createShares(this js.Value, i []js.Value) interface{} {
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
	bits := 2048
	p, err := rand.Prime(rand.Reader, bits/2) // move this out from wasm, it tooks too much time
	if err != nil {
		println(err.Error())
	}
	k, ok := new(big.Int).SetString(i[3].String(), 10)
	if !ok {
		println("error parsing parameter in position 3")
	}
	println("nNeededShares", nNeededShares.String())
	println("nShares", nShares.String())
	println("p", p.String())
	println("k (secret)", k.String())
	shares, err := shamirsecretsharing.Create(nNeededShares, nShares, p, k)
	if err != nil {
		println("error generating the shares")
		println(err.Error())
	}
	println("shares", shares)
	sharesStr := sharesToString(shares)
	println("sharesStr", sharesStr)
	return nil
}

func sharesToString(shares [][]*big.Int) []string {
	var printString string
	var s []string
	for i := 0; i < len(shares); i++ {
		s = append(s, shares[i][0].String())
		s = append(s, shares[i][1].String())
		printString = printString + "[" + shares[i][0].String() + ", " + shares[i][1].String() + "]\n"
		println(shares[i][0].String())
		println(shares[i][1].String())
	}
	js.Global().Get("document").Call("getElementById", "sharesResult").Set("innerHTML", printString)
	return s
}

func lagrangeInterpolation(this js.Value, i []js.Value) interface{} {
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
	return nil
}
