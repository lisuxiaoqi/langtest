package main

import (
	"fmt"
	"testing"
)

type Person struct {
	Name string
	Age  int
}

/*
v:<nil>
+v:<nil>
#v:(*int)(nil)
T:*int

v:{John 18}
+v:{Name:John Age:18}
#v:main.Person{Name:"John", Age:18}
T:main.Person
*/
func Test_Fmt(t *testing.T) {
	var m *int
	var i interface{} = m

	fmt.Printf("v:%v\n", i)
	fmt.Printf("+v:%+v\n", i)
	fmt.Printf("#v:%#v\n", i)
	fmt.Printf("T:%T\n", i)

	p := Person{"John", 18}
	//显示值，没有字段名
	fmt.Printf("v:%v\n", p)
	//显示值，加上字段名
	fmt.Printf("+v:%+v\n", p)
	//显示值，加上字段名，还有类型
	fmt.Printf("#v:%#v\n", p)
	//显示类型
	fmt.Printf("T:%T\n", p)
}
