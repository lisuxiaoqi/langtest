package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"github.com/stretchr/testify/assert"
	"math/big"
	"strings"
	"testing"
)

func TestJson2Map(t *testing.T) {
	raw := "{\"id\": 2, \"uid\": 50000003, \"address\": \"0xBC1C8431Ac4FDaF332bF83A4c7C372a65dC54c07\", \"signature\": \"0xf5509f8f8092c0193aad6ad6330940f969c3b2621a3ec039afcd5e965fc5102fe0a1d6e88d25709b3cb0d17e53644a3a460d7b33f83f75e9eb84aa1437df1105\"}"

	var mapResult map[string]interface{}
	//使用 json.Unmarshal(data []byte, v interface{})进行转换,返回 error 信息
	if err := json.Unmarshal([]byte(raw), &mapResult); err != nil {
		t.Fatal(err)
	}
	t.Log(mapResult["id"])

	//get id
	fmt.Printf("%T\n", mapResult["id"])
	if val, ok := mapResult["id"].(float64); ok {
		num := new(big.Int).SetUint64(uint64(val))
		fmt.Println("Big int number to int", num.Int64())
		fmt.Println("Big int number to string", num.String())
	}
}

func TestContains(t *testing.T) {
	err := errors.New("sql: no rows in result set")
	s := err.Error()
	assert.True(t, strings.Contains(s, "no rows"))
}
