package main

import (
	"bytes"
	"fmt"
	"github.com/stretchr/testify/assert"
	"testing"
)

// slice index是【)半开闭区间
func TestIndex(t *testing.T) {
	src := "12345678"

	fmt.Println(src[0:1])
	fmt.Println(src[7:8])
}

/*
slice是基于底层数组的一段描述，包括可见长度，index等。
在传递时非常特殊，可以理解为值传递。
在赋值时，底层的数组时引用，都指向相同的底层数组，但slice描述本身是值传递。
虽然相同的底层数组，但是slice的显示范围可能不同，导致输出不同
*/
func TestSliceAssign(t *testing.T) {
	s1 := make([]int, 10, 20)
	s1 = append(s1, 1)
	fmt.Println("S1", s1, len(s1), cap(s1))
	fmt.Printf("s1 addr %p\n", s1)

	s2 := s1
	s2 = append(s2, 2)
	fmt.Println("S1", s1, len(s1), cap(s1))
	fmt.Println("S2", s2, len(s2), cap(s2))
	fmt.Printf("s1 addr %p\n", s1)
	fmt.Printf("s2 addr %p\n", s2)
}

/*
测试copy的用法
*/
func TestCopy(t *testing.T) {
	var t1 [][]byte
	t1 = append(t1, []byte("hello"))
	t1 = append(t1, []byte("world"))
	assert.True(t, bytes.Equal(t1[0], []byte("hello")))
	assert.True(t, bytes.Equal(t1[1], []byte("world")))

	//copy时t2的length,注意，不是cap，必须大等于t1的length
	t2 := make([][]byte, len(t1))
	//以下情况都会失败：
	//var t2 [][]byte	未初始化
	//t2 := make([][]byte, 0, len(t1))	length为0，虽然cap足够
	//t2 := make([][]byte, len(t1)-1)	length小
	copy(t2, t1)
	assert.True(t, bytes.Equal(t2[0], []byte("hello")))
	assert.True(t, bytes.Equal(t2[1], []byte("world")))

	//nil slice之间copy不会出错
	var t3, t4 [][]byte
	copy(t3, t4)
}

func TestSlice(t *testing.T) {
	s1 := []int{0, 1, 2, 3}
	s2 := s1[1:3]
	//s2的cap为0， 一个slice的cap，是从slice对应的底层数组的index到底层数组的结尾
	fmt.Printf("s2: %v, cap(s2): %d\n", s2, cap(s2)) // 1

	s1[1] = 10
	fmt.Printf("s1: %v, s2: %v\n", s1, s2) // 2

	//slice在append的时候，会导致重新分配内存，因此可能不会影响其他slice
	s2 = append(s2, 100)
	s2 = append(s2, 200)
	fmt.Printf("s1: %v, s2: %v\n", s1, s2) // 3
}

// 测试slice的去重
func TestSliceUniq(t *testing.T) {
	arr := []int{1, 1, 2, 2, 3, 3, 4}

	b := []int{}
	setB := make(map[int]struct{})
	for _, v := range arr {
		if _, ok := setB[v]; !ok {
			b = append(b, v)
			setB[v] = struct{}{}
		}
	}
	t.Log(b)
}
