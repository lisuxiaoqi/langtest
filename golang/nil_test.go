package main

import (
	"fmt"
	"testing"
)

/*
读取nil不会panic，只有操作nil，比如写入，读取nil的属性时会panic
*/
func TestMapNil(t *testing.T) {
	var m map[int]int
	//读取nil不会panic
	fmt.Println(m[0])
	//向nil写入会panic
	m[0] = 1
	fmt.Println(m[0])
}

type SA struct {
}

func (s *SA) foo() {
	fmt.Println("foo get called")
}

/*
	当一个方法没有试图访问 nil 接收者的字段或调用它的其他方法时，调用会正常进行，不会引发 panic。

只有当你尝试使用这个 nil 值的字段或调用其他需要实际结构体实例的方法时，才会引发 panic

	Go 允许 nil 指针调用其方法。换句话说，如果一个方法是基于指针接收者定义的，你可以通过 nil 指针调用它。

nil 仅仅意味着这个指针没有指向有效的结构体数据，但你仍然可以调用这个指针的接收者方法
*/
func TestStructNil(t *testing.T) {
	var s *SA
	if s == nil {
		fmt.Println("s is nil")
	}
	s.foo()
}

func TestSliceNil(t *testing.T) {
	//nil slice可以使用len
	var b []byte
	t.Log(len(b))

	var i interface{}
	i = b
	//nil的类型能正常赋值给interface
	//interface被赋值后不是nil
	switch i.(type) {
	case []byte:
		t.Log("type []byte", "b is nil", b == nil, "i is not nil", i == nil)
	}
}
