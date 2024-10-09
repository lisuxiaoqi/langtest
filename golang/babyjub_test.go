package main

import (
	"encoding/hex"
	"fmt"
	"github.com/iden3/go-iden3-crypto/babyjub"
	"github.com/iden3/go-iden3-crypto/utils"
	"github.com/stretchr/testify/assert"
	"math/big"
	"testing"
)

/*
测试Poseidon签名，验签，压缩，解压缩
*/
func TestSignVerifyPoseidon(t *testing.T) {
	//创建随机密钥
	privK := babyjub.NewRandPrivKey()
	pubK := privK.Public()

	//准备msg
	msgBuf, err := hex.DecodeString("00010203040506070809")
	if err != nil {
		panic(err)
	}
	msg := utils.SetBigIntFromLEBytes(new(big.Int), msgBuf)

	//verify signature
	sig := privK.SignPoseidon(msg)

	ok := pubK.VerifyPoseidon(msg, sig)
	assert.Equal(t, true, ok)

	//decompress without error
	sigBuf := sig.Compress()
	sig2, err := new(babyjub.Signature).Decompress(sigBuf)
	assert.Equal(t, nil, err)

	//decompressed result can pass verification
	ok = pubK.VerifyPoseidon(msg, sig2)
	assert.Equal(t, true, ok)
}

// 测试key与string转化
func TestKeyString(t *testing.T) {
	//创建随机密钥
	privK := babyjub.NewRandPrivKey()
	pubK := privK.Public()

	//byte to string
	privKeyStr := hex.EncodeToString(privK[:])
	fmt.Println("PrivKey:", privKeyStr)

	//注意pubK的string，是先把原始的pubK压缩成PublicKeyComp
	//再编码成hex
	pubKeyCompStr := pubK.String()
	fmt.Println("PubKeyComp:", pubKeyCompStr)

	//string to byte
	var privK2 babyjub.PrivateKey
	_, err := hex.Decode(privK2[:], []byte(privKeyStr))
	assert.Equal(t, nil, err)

	//先把pubKeyCompStr反序列化为PublicKeyComp
	var pubK2Comp babyjub.PublicKeyComp
	err = pubK2Comp.UnmarshalText([]byte(pubKeyCompStr))
	assert.Equal(t, nil, err)
	//再把PublicKeyComp解压缩成pubKey
	var pubK2 *babyjub.PublicKey
	pubK2, err = pubK2Comp.Decompress()
	assert.Equal(t, nil, err)

	//准备msg
	msgBuf, err := hex.DecodeString("00010203040506070809")
	if err != nil {
		panic(err)
	}
	msg := utils.SetBigIntFromLEBytes(new(big.Int), msgBuf)

	//用从字符串反序列化的privK2签名
	sig := privK2.SignPoseidon(msg)

	//能被原始的pubK验签
	ok := pubK.VerifyPoseidon(msg, sig)
	assert.Equal(t, true, ok)

	//用原始的privK签名
	sig2 := privK.SignPoseidon(msg)

	//能被反序列化的pubK2签名
	ok = pubK2.VerifyPoseidon(msg, sig2)
	assert.Equal(t, true, ok)
}

// 测试签名与string转换
func TestSignatureString(t *testing.T) {
	privString := "28057e2ad1a08c01986c20e625716cbaa8624fdcf3506bd12905e8fa3112d179"
	var privK babyjub.PrivateKey
	_, err := hex.Decode(privK[:], []byte(privString))
	assert.Equal(t, nil, err)

	//生成签名
	msgBuf, err := hex.DecodeString("00010203040506070809")
	if err != nil {
		panic(err)
	}
	msg := utils.SetBigIntFromLEBytes(new(big.Int), msgBuf)
	sig := privK.SignPoseidon(msg)

	//签名压缩后转化为String
	sigBuf := sig.Compress()
	sigString := hex.EncodeToString(sigBuf[:])
	fmt.Println("Signature:", sigString)

	assert.Equal(t, ""+
		"b342733ba6cf2225b9a4e946d0f9c46013be3172d53f2dd2cfd5f98f783c0309a08"+
		"cbfebb966dc1fd57b0ce6dc800961f82eb86615fcc72cb8bb5bdcb990aa04",
		sigString)

	//把String转化回签名
	sigBuf2, err := hex.DecodeString(sigString)
	var sigBufComp babyjub.SignatureComp
	copy(sigBufComp[:], sigBuf2)
	sig2, err := new(babyjub.Signature).Decompress(sigBufComp)
	assert.Equal(t, nil, err)

	//验签
	pubK := privK.Public()
	ok := pubK.VerifyPoseidon(msg, sig2)
	assert.Equal(t, ok, true)

}
