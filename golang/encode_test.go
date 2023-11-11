package main

import (
	"encoding/hex"
	"fmt"
	"testing"
)

/*
测试16进制
*/
func TestByteSwap(t *testing.T) {
	s := "abcdef"

	//encode string to hex:把string的每个char变成hex(ascii格式)的string
	//abcdef:0x616263646566
	sHex := hex.EncodeToString([]byte(s))
	fmt.Printf("Encode String to hex:0x%s\n", sHex)

	//decode hex to string
	//把ascii码格式，解码为string
	sByte, _ := hex.DecodeString(sHex)
	fmt.Println("Deocde hex to String:", string(sByte))

	//swap byte
	swapedByte := SwapEndianness(sByte)
	fmt.Println("Swapped hex to String:", string(swapedByte))
}

func SwapEndianness(b []byte) []byte {
	o := make([]byte, len(b))
	for i := range b {
		o[len(b)-1-i] = b[i]
	}
	return o
}
