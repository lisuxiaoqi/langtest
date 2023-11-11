package main

import (
	"fmt"
	"testing"
)

// slice index是【)半开闭区间
func TestIndex(t *testing.T) {
	src := "12345678"

	fmt.Println(src[0:1])
	fmt.Println(src[7:8])
}
