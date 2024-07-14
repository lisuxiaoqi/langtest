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
