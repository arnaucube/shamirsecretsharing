package shamirsecretsharing

import (
	"bytes"
	"math/big"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCreate(t *testing.T) {
	k, ok := new(big.Int).SetString("12345678901234567890123456789012345678", 10)
	assert.True(t, ok)

	// 2 ** 127 - 1
	p, ok := new(big.Int).SetString("170141183460469231731687303715884105727", 10)
	assert.True(t, ok)

	nShares := big.NewInt(int64(6))
	nNeededShares := big.NewInt(int64(3))
	shares, err := Create(
		nNeededShares,
		nShares,
		p,
		k)
	assert.Nil(t, err)

	//generate sharesToUse
	var sharesToUse [][]*big.Int
	sharesToUse = append(sharesToUse, shares[2])
	sharesToUse = append(sharesToUse, shares[1])
	sharesToUse = append(sharesToUse, shares[0])
	secr := LagrangeInterpolation(p, sharesToUse)

	// fmt.Print("original secret: ")
	// fmt.Println(k)
	// fmt.Print("p: ")
	// fmt.Println(p)
	// fmt.Print("shares: ")
	// fmt.Println(shares)
	// fmt.Print("recovered secret result: ")
	// fmt.Println(secr)
	if !bytes.Equal(k.Bytes(), secr.Bytes()) {
		t.Errorf("reconstructed secret not correspond to original secret")
	}
	assert.Equal(t, k, secr)
}
