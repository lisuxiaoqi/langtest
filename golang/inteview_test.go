package main

import (
	"fmt"
	"testing"
)

/*
s2: [1 2], cap(s2): 3
s1: [0 10 2 3], s2: [10 2]
s1: [0 10 2 100], s2: [10 2 100 200]
*/
func Test_Slice(t *testing.T) {
	s1 := []int{0, 1, 2, 3}
	s2 := s1[1:3]
	fmt.Printf("s2: %v, cap(s2): %d\n", s2, cap(s2)) // 1

	s1[1] = 10
	fmt.Printf("s1: %v, s2: %v\n", s1, s2) // 2

	s2 = append(s2, 100)
	s2 = append(s2, 200)
	fmt.Printf("s1: %v, s2: %v\n", s1, s2) // 3
}

// 输出3，i仅保留了最后一个值
func Test_For(t *testing.T) {
	funcs := []func(){}
	for i := 0; i < 3; i++ {
		funcs = append(funcs, func() { fmt.Println(i) })
	}
	for _, f := range funcs {
		f()
	}
}

/*
result is 11
defer在return之前执行，整体执行流程类似于：

	result = 10
	result++
	return
*/
func testDefer() (result int) {
	defer func() {
		result++
	}()
	return 10
}

func Test_Defer(t *testing.T) {
	fmt.Println("result is", testDefer())
}

/*
m is nil
i is not nil, (*int)(nil)
*/
func Test_Nil(t *testing.T) {
	var m *int
	var i interface{} = m

	if m == nil {
		fmt.Println("m is nil")
	} else {
		fmt.Printf("m is not nil")
	}

	if i == nil {
		fmt.Println("i is nil")
	} else {
		fmt.Printf("i is not nil")
	}
}

func Test_MapNil(t *testing.T) {
	var m map[int]int
	//read no crash
	fmt.Println(m[0])

	//write will crash
	//m[0] = 1
	//fmt.Println(m[0])
}
