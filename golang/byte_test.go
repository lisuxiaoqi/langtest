package main

import (
	"encoding/hex"
	"fmt"
	"testing"
)

/*
测试16进制
*/
func TestByteConvert(t *testing.T) {
	s := "hello world"

	//string to byte
	b := []byte(s)
	fmt.Println(b)

	//byte to string
	sFromByte := string(b)
	fmt.Println(sFromByte)

	//byte to hex string
	sHexFromByte := hex.EncodeToString(b)
	fmt.Println(sHexFromByte)

	//hex string to byte
	hexString := "68656c6c6f20776f726c64"
	byteFromHexString, _ := hex.DecodeString(hexString)
	fmt.Println(byteFromHexString)
	stringFromHexString := string(byteFromHexString)
	fmt.Println(stringFromHexString)
}
