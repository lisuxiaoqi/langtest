package main

import (
	"encoding/hex"
	"fmt"
	"golang.org/x/crypto/sha3"
	"testing"
)

/*
测试16进制
*/
func TestKec256Hash(t *testing.T) {
	hash := sha3.NewLegacyKeccak256()

	//expected: 65a7ed542fb37fe237fdfbdd70b31598523fe5b32879e307bae27a0bd9581c08
	msg := "systemconfig.unsafeblocksigner"
	hash.Write([]byte(msg))
	buf := hash.Sum(nil)

	fmt.Println(hex.EncodeToString(buf))
}
