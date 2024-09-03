package main

import (
	"fmt"
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

// 测试bigInt计算后，赋值的是指针
func TestChange(t *testing.T) {
	a := new(big.Int).SetUint64(10)
	b := new(big.Int).SetUint64(20)
	fmt.Println("Before change", "a:", a, "b:", b)

	//a和b都是*big.Int,指向相同的地址
	b = a.Mul(a, b)
	fmt.Println("After change", "a:", a, "b:", b)
}

// bigInt nil will crash
func TestNil(t *testing.T) {
	var big *big.Int
	fmt.Println("nil big int to uint64", big.Uint64())

}
