package main

import (
	"github.com/stretchr/testify/require"
	"math/big"
	"testing"
)

// 测试bigInt的左右移位
func TestShift(t *testing.T) {
	b := new(big.Int).SetUint64(2)
	b.Lsh(b, 2)
	require.Equal(t, new(big.Int).SetUint64(8), b)
	b.Rsh(b, 1)
	require.Equal(t, new(big.Int).SetUint64(4), b)
}
