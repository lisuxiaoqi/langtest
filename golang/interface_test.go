package main

import (
	"fmt"
	"reflect"
	"testing"
)

type IA interface {
	Itype()
}

type Apple struct{}

func (s *Apple) Itype() {
	println("Type Apple")
}

func TestInterfaceType(t *testing.T) {
	var i IA = &Apple{}
	fmt.Println(reflect.TypeOf(i))
}
