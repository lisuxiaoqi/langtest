package main

import (
	"fmt"
	"math"
	"testing"
)

/*
初始的bool是false
*/
func TestDefaultBoolean(t *testing.T) {
	var b bool
	fmt.Println("b:", b)
}

/*
int除以int，结果还是int，舍掉小数部分，不会向上取整
*/
func TestIntDivision(t *testing.T) {
	i := 100 * uint64(30)
	f := i / 70
	fmt.Println("f:", f)
}

// if语句只进第一个为true的语句，其他的不执行
func TestIf(t *testing.T) {
	i := 2
	if i > 1 {
		fmt.Println("i > 1")
	} else if i < 3 {
		fmt.Println("i<3")
	}
}

/*
当struct的函数是值传递时，不会改变struct的状态，而是copy了struct
*/
type S struct {
	f int
}

// s是copy，改变s不会改变调用者的状态
func (s S) UpdateFiled(val int) S {
	s.f = val
	return s
}

// s是copy，改变s不会改变调用者的状态
func (s S) UpdateFiledWNoReturn(val int) {
	s.f = val
}

func TestAssign(t *testing.T) {
	s1 := S{f: 3}
	fmt.Println("s1:", s1)

	s1.UpdateFiled(13)
	fmt.Println("s1:", s1)

	s1.UpdateFiledWNoReturn(13)
	fmt.Println("s1:", s1)

	s2 := s1.UpdateFiled(23)
	fmt.Println("s1:", s1, "s2:", s2)
}

/*
map是引用传递，改变引用的值会改变原始map
*/
func TestMapAssign(t *testing.T) {
	s1 := make(map[string]int)
	s1["key1"] = 1
	fmt.Println("S1", s1)

	s2 := s1
	s2["key1"] = 2
	fmt.Println("S1", s1)
	fmt.Println("S2", s2)
}

func TestTemp(t *testing.T) {
	var orderId int = 548853144429
	newOrderId := uint32(orderId)

	fmt.Println("oldOrderId", orderId,
		"maxUint32", math.MaxUint32,
		"newOrderId:", newOrderId)
}
